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
use crate::events::DeadAdventurerReward;
use crate::events::EventPayload;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::relics::get_relic;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::Rewards;
use crate::types::RoomKind;
use crate::utils::add_relic_reward_for_roll;
use crate::utils::has_relic;
use crate::utils::push_entity;
use crate::utils::roll_card_rewards;

pub fn process_effect_reward_roll_combat(
    state: &mut GameState,
    room_kind: RoomKind,
    escaped: bool,
) {
    // Select roll parameters according to `RoomKind`; event combats inject their
    // event-specific extras (a fixed relic and a bespoke gold range)
    let (gold_range, relic_thresholds, event_relic) = match room_kind {
        RoomKind::CombatMonster => (
            if escaped {
                None
            } else {
                Some((GOLD_MONSTER_MIN, GOLD_MONSTER_MAX))
            },
            None,
            None,
        ),
        RoomKind::CombatElite => (
            Some((GOLD_ELITE_MIN, GOLD_ELITE_MAX)),
            Some((RELIC_TIER_TH_COMMON, RELIC_TIER_TH_UNCOMMON)),
            None,
        ),
        RoomKind::EventRoom => {
            let event = state
                .event
                .expect("event-room combat requires an active event");
            match event.payload {
                EventPayload::Mushrooms => (Some((20, 30)), None, Some(RelicName::OddMushroom)),
                EventPayload::DeadAdventurer {
                    rewards,
                    rewards_len,
                    ..
                } => {
                    // Un-found rewards fold into the fight's loot
                    let unfound_rewards = &rewards[..rewards_len as usize];
                    let gold_extra = unfound_rewards
                        .iter()
                        .filter(|&&r| r == DeadAdventurerReward::Gold)
                        .count() as u16
                        * 30;
                    let relic_unfound = unfound_rewards.contains(&DeadAdventurerReward::Relic);
                    (
                        Some((25 + gold_extra, 35 + gold_extra)),
                        relic_unfound.then_some((RELIC_TIER_TH_COMMON, RELIC_TIER_TH_UNCOMMON)),
                        None,
                    )
                }
                payload => unreachable!("combat reward in non-combat event: {payload:?}"),
            }
        }
        _ => unreachable!(
            "RewardRollCombat with non-combat room_kind: {:?}",
            room_kind
        ),
    };

    // Roll Cards
    let mut id_cards: Vec<usize> = Vec::with_capacity(MAX_COMBAT_CARD_REWARD);
    roll_card_rewards(
        state.id_character,
        &mut state.entities,
        &mut state.rng,
        &mut id_cards,
        &state.id_relics,
    );

    // Roll Relic (only for Elite combats)
    let mut id_relic = relic_thresholds.map(|(th_common, th_uncommon)| {
        let roll = state.rng.random_range(0..100) as u8;
        add_relic_reward_for_roll(
            roll,
            th_common,
            th_uncommon,
            &state.id_relics,
            &mut state.entities,
            &mut state.rng,
        )
    });

    // Event-injected relic; Circlet substitutes when it is already owned
    if let Some(name) = event_relic {
        let name = if has_relic(&state.id_relics, name) {
            RelicName::Circlet
        } else {
            name
        };
        id_relic = Some(push_entity(&mut state.entities, get_relic(name)));
    }

    // Roll Potions
    // White Beast Statue: guaranteed drop, bypassing the drifting chance roll
    let potion_drops = has_relic(&state.id_relics, RelicName::WhiteBeastStatue)
        || roll_potion_drop(&mut state.rng, &mut state.potion_drop_mod);
    let mut id_potions: Vec<usize> = Vec::with_capacity(1);
    if potion_drops {
        let name = get_random_potion_name(&mut state.rng, false);
        let id = push_entity(&mut state.entities, get_potion(name));
        id_potions.push(id);
    }

    // Roll gold
    let gold = gold_range.map(|(min, max)| {
        let gold = state.rng.random_range(min..=max);
        // Golden Idol: 25% bonus rounded half-up on combat rewards only
        if has_relic(&state.id_relics, RelicName::GoldenIdol) {
            gold + (gold + 2) / 4
        } else {
            gold
        }
    });

    state.mode = Mode::Reward(Rewards {
        id_cards,
        id_relic,
        id_potions,
        gold,
    });
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
