use rand::Rng;

use crate::consts::GOLD_ELITE_MAX;
use crate::consts::GOLD_ELITE_MIN;
use crate::consts::GOLD_MONSTER_MAX;
use crate::consts::GOLD_MONSTER_MIN;
use crate::consts::POTION_DROP_CHANCE_BASE;
use crate::consts::POTION_DROP_CHANCE_MOD_HIT;
use crate::consts::POTION_DROP_CHANCE_MOD_MAX;
use crate::consts::POTION_DROP_CHANCE_MOD_MIN;
use crate::consts::POTION_DROP_CHANCE_MOD_MISS;
use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::game::GameState;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::types::Screen;
use crate::utils::add_relic_reward_for_roll;
use crate::utils::push_entity;
use crate::utils::roll_card_rewards;

pub fn process_effect_reward_roll_combat(state: &mut GameState, room_kind: RoomKind) {
    let escaped = if matches!(state.screen, Screen::Combat) {
        state.this_combat_escaped
    } else {
        false
    };

    let (gold_range, relic_thresholds) = match room_kind {
        RoomKind::CombatMonster => (
            if escaped {
                None
            } else {
                Some((GOLD_MONSTER_MIN, GOLD_MONSTER_MAX))
            },
            None,
        ),
        RoomKind::CombatElite => (
            Some((GOLD_ELITE_MIN, GOLD_ELITE_MAX)),
            Some((RELIC_TIER_TH_COMMON, RELIC_TIER_TH_UNCOMMON)),
        ),
        _ => unreachable!(
            "RewardRollCombat with non-combat room_kind: {:?}",
            room_kind
        ),
    };

    roll_card_rewards(
        state.id_character,
        &mut state.entities,
        &mut state.rng,
        &mut state.reward_id_cards,
    );
    state.reward_id_relic = relic_thresholds.map(|(th_c, th_u)| {
        let roll = state.rng.random_range(0..100) as u8;
        add_relic_reward_for_roll(
            roll,
            th_c,
            th_u,
            &state.id_relics,
            &mut state.entities,
            &mut state.rng,
        )
    });
    state.reward_id_potion =
        roll_potion_drop(&mut state.rng, &mut state.potion_drop_mod).then(|| {
            let name = get_random_potion_name(&mut state.rng, false);
            push_entity(&mut state.entities, get_potion(name))
        });
    state.reward_gold = gold_range.map(|(min, max)| {
        let gold = state.rng.random_range(min..=max);
        // GoldenIdol: +25% rounded half-up on combat rewards only
        if state.id_relics[RelicName::GoldenIdol as usize].is_some() {
            gold + (gold + 2) / 4
        } else {
            gold
        }
    });

    state.screen = Screen::Reward;
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
