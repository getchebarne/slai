use rand::Rng;

use crate::consts::CHEST_LARGE;
use crate::consts::CHEST_MEDIUM;
use crate::consts::CHEST_SMALL;
use crate::consts::ChestParams;
use crate::game::GameState;
use crate::types::Screen;
use crate::types::ChestKind;
use crate::utils::add_relic_reward_for_roll;

pub fn process_effect_reward_roll_chest(state: &mut GameState, chest_kind: ChestKind) {
    let chest_params = match chest_kind {
        ChestKind::Small => CHEST_SMALL,
        ChestKind::Medium => CHEST_MEDIUM,
        ChestKind::Large => CHEST_LARGE,
    };

    let roll = state.rng.random_range(0..100) as u8;
    state.reward_gold = if roll < chest_params.gold_chance {
        Some(roll_gold_amount(&mut state.rng, chest_params))
    } else {
        None
    };
    state.reward_id_relic = Some(add_relic_reward_for_roll(
        roll,
        chest_params.th_common,
        chest_params.th_uncommon,
        &state.id_relics,
        &mut state.entities,
        &mut state.rng,
    ));
    state.reward_id_cards.clear();
    state.reward_id_potion = None;

    state.active = Screen::Reward;
}

fn roll_gold_amount(rng: &mut impl Rng, chest_params: ChestParams) -> u16 {
    let base = chest_params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}
