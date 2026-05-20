use rand::Rng;
use strum::EnumCount;

use crate::consts::ELITE_TH_COMMON;
use crate::consts::ELITE_TH_UNCOMMON;
use crate::consts::GOLD_ELITE_MAX;
use crate::consts::GOLD_ELITE_MIN;
use crate::consts::GOLD_MONSTER_MAX;
use crate::consts::GOLD_MONSTER_MIN;
use crate::consts::POTION_DROP_CHANCE_BASE;
use crate::consts::POTION_DROP_CHANCE_MOD_HIT;
use crate::consts::POTION_DROP_CHANCE_MOD_MAX;
use crate::consts::POTION_DROP_CHANCE_MOD_MIN;
use crate::consts::POTION_DROP_CHANCE_MOD_MISS;
use crate::entity::Entity;
use crate::potions::get_potion;
use crate::potions::get_random_potion;
use crate::types::RelicName;
use crate::types::RewardState;
use crate::types::RoomKind;
use crate::utils::add_relic_reward_for_roll;
use crate::utils::roll_card_rewards;

pub fn process_effect_reward_roll_combat(
    room_kind: RoomKind,
    id_character: usize,
    id_relics: &[Option<usize>; RelicName::COUNT],
    escaped_this_combat: bool,
    potion_drop_mod: &mut i8,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> RewardState {
    let (gold_range, relic_thresholds) = match room_kind {
        RoomKind::CombatMonster => (
            if escaped_this_combat {
                None
            } else {
                Some((GOLD_MONSTER_MIN, GOLD_MONSTER_MAX))
            },
            None,
        ),
        RoomKind::CombatElite => (
            Some((GOLD_ELITE_MIN, GOLD_ELITE_MAX)),
            Some((ELITE_TH_COMMON, ELITE_TH_UNCOMMON)),
        ),
        _ => unreachable!(
            "RewardRollCombat with non-combat room_kind: {:?}",
            room_kind
        ),
    };

    let id_cards = roll_card_rewards(id_character, entities, rng);
    let id_relic = relic_thresholds.map(|(th_c, th_u)| {
        let roll = rng.random_range(0..100) as u8;
        add_relic_reward_for_roll(roll, th_c, th_u, id_relics, entities, rng)
    });
    let id_potion = roll_potion_drop(rng, potion_drop_mod).then(|| {
        let name = get_random_potion(rng, false);
        let id = entities.len();
        entities.push(get_potion(name));
        id
    });
    let gold = gold_range.map(|(min, max)| rng.random_range(min..=max));

    RewardState {
        id_cards,
        id_relic,
        id_potion,
        gold,
    }
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
