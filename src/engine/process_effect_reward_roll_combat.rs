use rand::Rng;

use crate::consts::GOLD_ELITE_MAX;
use crate::consts::GOLD_ELITE_MIN;
use crate::consts::GOLD_MONSTER_MAX;
use crate::consts::GOLD_MONSTER_MIN;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::consts::POTION_DROP_CHANCE_BASE;
use crate::consts::POTION_DROP_CHANCE_MOD_HIT;
use crate::consts::POTION_DROP_CHANCE_MOD_MAX;
use crate::consts::POTION_DROP_CHANCE_MOD_MIN;
use crate::consts::POTION_DROP_CHANCE_MOD_MISS;
use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Amount;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::relics::get_relic;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::add_relic_reward_for_roll;
use crate::utils::card_reward_count;
use crate::utils::has_relic;
use crate::utils::push_entity;
use crate::utils::roll_card_rewards;

pub fn process_effect_reward_roll_combat(
    state: &mut GameState,
    room_kind: RoomKind,
    escaped: bool,
    event_gold: Option<Amount>,
    event_relic: Option<RelicName>,
    event_relic_roll: bool,
) {
    // Select roll parameters according to `RoomKind`; event combats inject their
    // event-specific extras
    let (gold_amount, relic_thresholds, event_relic) = match room_kind {
        RoomKind::CombatMonster => (
            (!escaped).then_some(Amount::Range {
                min: GOLD_MONSTER_MIN,
                max: GOLD_MONSTER_MAX,
            }),
            None,
            None,
        ),
        RoomKind::CombatElite => (
            Some(Amount::Range {
                min: GOLD_ELITE_MIN,
                max: GOLD_ELITE_MAX,
            }),
            Some((RELIC_TIER_TH_COMMON, RELIC_TIER_TH_UNCOMMON)),
            None,
        ),
        RoomKind::EventRoom => (
            Some(event_gold.expect("event fight without stamped loot")),
            event_relic_roll.then_some((RELIC_TIER_TH_COMMON, RELIC_TIER_TH_UNCOMMON)),
            event_relic,
        ),
        _ => unreachable!(
            "RewardRollCombat with non-combat room_kind: {:?}",
            room_kind
        ),
    };

    // Roll Cards; Prayer Wheel adds a second bundle on normal fights
    let bundle_count = if room_kind == RoomKind::CombatMonster
        && has_relic(&state.id_relics, RelicName::PrayerWheel)
    {
        2
    } else {
        1
    };
    let cards_per_bundle = card_reward_count(&state.id_relics);
    let mut id_card_bundles: Vec<Vec<usize>> = Vec::with_capacity(bundle_count);
    for _ in 0..bundle_count {
        let mut id_cards: Vec<usize> = Vec::with_capacity(MAX_COMBAT_CARD_REWARD);
        roll_card_rewards(
            state.id_character,
            &mut state.entities,
            &mut state.rng,
            &mut id_cards,
            &state.id_relics,
            cards_per_bundle,
        );
        id_card_bundles.push(id_cards);
    }

    // Roll Relic (only for Elite combats)
    let mut id_relics_reward: Vec<usize> = Vec::new();
    if let Some((th_common, th_uncommon)) = relic_thresholds {
        let roll = state.rng.random_range(0..100) as u8;
        let id_first = add_relic_reward_for_roll(
            roll,
            th_common,
            th_uncommon,
            &state.id_relics,
            &mut state.entities,
            &mut state.rng,
        );
        id_relics_reward.push(id_first);

        // Black Star: elites drop a second relic with an independent tier roll;
        // exclude the first pick so the pair can't collide
        if has_relic(&state.id_relics, RelicName::BlackStar) {
            let mut taken = state.id_relics;
            taken[state.entities[id_first].relic_name as usize] = Some(id_first);
            let roll = state.rng.random_range(0..100) as u8;
            id_relics_reward.push(add_relic_reward_for_roll(
                roll,
                th_common,
                th_uncommon,
                &taken,
                &mut state.entities,
                &mut state.rng,
            ));
        }
    }

    // Event-injected relic; Circlet substitutes when it is already owned
    if let Some(name) = event_relic {
        let name = if has_relic(&state.id_relics, name) {
            RelicName::Circlet
        } else {
            name
        };
        id_relics_reward.push(push_entity(&mut state.entities, get_relic(name)));
    }

    // Roll Potions
    // White Beast Statue: guaranteed drop, bypassing the drifting chance roll
    // Sozu: no potion drops at all (skips the roll and its chance drift)
    let potion_drops = !has_relic(&state.id_relics, RelicName::Sozu)
        && (has_relic(&state.id_relics, RelicName::WhiteBeastStatue)
            || roll_potion_drop(&mut state.rng, &mut state.potion_drop_mod));
    let mut id_potions: Vec<usize> = Vec::with_capacity(1);
    if potion_drops {
        let name = get_random_potion_name(&mut state.rng, false);
        let id = push_entity(&mut state.entities, get_potion(name));
        id_potions.push(id);
    }

    // Roll gold
    let gold = if let Some(amount) = gold_amount {
        let mut rolled = match amount {
            Amount::Absolute(amount) => amount,
            Amount::Range { min, max } => state.rng.random_range(min..=max),
            _ => unreachable!("Reward gold only resolves Absolute or Range"),
        };
        // Golden Idol: 25% bonus rounded half-up on combat rewards only
        if has_relic(&state.id_relics, RelicName::GoldenIdol) {
            rolled += (rolled + 2) / 4;
        }
        Some(rolled)
    } else {
        None
    };

    *state.mode_stack.last_mut().expect("mode stack never empty") = Mode::Reward {
        reward_id_cards: id_card_bundles,
        reward_id_relics: id_relics_reward,
        reward_id_potions: id_potions,
        reward_gold: gold,
    };
}

// +10 on miss, -10 on hit; clamps to [-30, +60] ([10%, 100%])
fn roll_potion_drop(rng: &mut impl Rng, potion_drop_mod: &mut i8) -> bool {
    let roll = rng.random_range(0..100) as u8;
    let chance = (POTION_DROP_CHANCE_BASE as i16 + *potion_drop_mod as i16).clamp(0, 100) as u8;

    if roll < chance {
        *potion_drop_mod = (*potion_drop_mod + POTION_DROP_CHANCE_MOD_HIT)
            .clamp(POTION_DROP_CHANCE_MOD_MIN, POTION_DROP_CHANCE_MOD_MAX);
        true
    } else {
        *potion_drop_mod = (*potion_drop_mod + POTION_DROP_CHANCE_MOD_MISS)
            .clamp(POTION_DROP_CHANCE_MOD_MIN, POTION_DROP_CHANCE_MOD_MAX);
        false
    }
}
