use rand::Rng;
use strum::EnumCount;

use crate::consts::CHEST_LARGE;
use crate::consts::CHEST_MEDIUM;
use crate::consts::CHEST_SMALL;
use crate::consts::ChestParams;
use crate::entity::Entity;
use crate::types::ChestKind;
use crate::types::Phase;
use crate::types::RelicName;
use crate::utils::add_relic_reward_for_roll;

pub fn process_effect_reward_roll_chest(
    chest_kind: ChestKind,
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> Option<Phase> {
    let chest_params = match chest_kind {
        ChestKind::Small => CHEST_SMALL,
        ChestKind::Medium => CHEST_MEDIUM,
        ChestKind::Large => CHEST_LARGE,
    };

    // Shared roll: gold-yes/no and relic-tier share the same draw
    let roll = rng.random_range(0..100) as u8;
    let gold = if roll < chest_params.gold_chance {
        Some(roll_gold_amount(rng, chest_params))
    } else {
        None
    };
    let id_relic = Some(add_relic_reward_for_roll(
        roll,
        chest_params.th_common,
        chest_params.th_uncommon,
        id_relics,
        entities,
        rng,
    ));

    Some(Phase::Reward {
        id_cards: Vec::new(),
        id_relic,
        id_potion: None,
        gold,
    })
}

fn roll_gold_amount(rng: &mut impl Rng, chest_params: ChestParams) -> u16 {
    let base = chest_params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}
