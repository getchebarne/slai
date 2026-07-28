use rand::Rng;

use crate::consts::CHEST_LARGE_GOLD_BASE;
use crate::consts::CHEST_LARGE_GOLD_CHANCE;
use crate::consts::CHEST_LARGE_TH_COMMON;
use crate::consts::CHEST_LARGE_TH_UNCOMMON;
use crate::consts::CHEST_MEDIUM_GOLD_BASE;
use crate::consts::CHEST_MEDIUM_GOLD_CHANCE;
use crate::consts::CHEST_MEDIUM_TH_COMMON;
use crate::consts::CHEST_MEDIUM_TH_UNCOMMON;
use crate::consts::CHEST_SMALL_GOLD_BASE;
use crate::consts::CHEST_SMALL_GOLD_CHANCE;
use crate::consts::CHEST_SMALL_TH_COMMON;
use crate::consts::CHEST_SMALL_TH_UNCOMMON;
use crate::consts::MATRYOSHKA_TH_COMMON;
use crate::consts::MATRYOSHKA_TH_UNCOMMON;
use crate::game::GameState;
use crate::types::ChestKind;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::add_relic_reward_for_roll;

#[derive(Debug, Clone, Copy)]
struct ChestParams {
    gold_chance: u8,
    gold_base: u16,
    th_common: u8,
    th_uncommon: u8,
}

pub fn process_effect_reward_roll_chest(state: &mut GameState, chest_kind: ChestKind) {
    let chest_params = match chest_kind {
        ChestKind::Small => ChestParams {
            gold_chance: CHEST_SMALL_GOLD_CHANCE,
            gold_base: CHEST_SMALL_GOLD_BASE,
            th_common: CHEST_SMALL_TH_COMMON,
            th_uncommon: CHEST_SMALL_TH_UNCOMMON,
        },
        ChestKind::Medium => ChestParams {
            gold_chance: CHEST_MEDIUM_GOLD_CHANCE,
            gold_base: CHEST_MEDIUM_GOLD_BASE,
            th_common: CHEST_MEDIUM_TH_COMMON,
            th_uncommon: CHEST_MEDIUM_TH_UNCOMMON,
        },
        ChestKind::Large => ChestParams {
            gold_chance: CHEST_LARGE_GOLD_CHANCE,
            gold_base: CHEST_LARGE_GOLD_BASE,
            th_common: CHEST_LARGE_TH_COMMON,
            th_uncommon: CHEST_LARGE_TH_UNCOMMON,
        },
    };

    let roll = state.rng.random_range(0..100) as u8;
    let gold = if roll < chest_params.gold_chance {
        Some(roll_gold_amount(&mut state.rng, chest_params))
    } else {
        None
    };
    let mut id_relics_reward: Vec<usize> = Vec::with_capacity(2);

    // Matryoshka: the next 2 chests hold an extra relic (75% Common / 25% Uncommon),
    // staged before the chest's own relic
    if let Some(id) = state.id_relics[RelicName::Matryoshka as usize]
        && state.entities[id].relic_counter > 0
    {
        let relic = &mut state.entities[id];
        relic.relic_counter -= 1;
        relic.relic_used_up = relic.relic_counter == 0;
        let extra_roll = state.rng.random_range(0..100) as u8;
        id_relics_reward.push(add_relic_reward_for_roll(
            extra_roll,
            MATRYOSHKA_TH_COMMON,
            MATRYOSHKA_TH_UNCOMMON,
            &state.id_relics,
            &mut state.entities,
            &mut state.rng,
        ));
    }

    id_relics_reward.push(add_relic_reward_for_roll(
        roll,
        chest_params.th_common,
        chest_params.th_uncommon,
        &state.id_relics,
        &mut state.entities,
        &mut state.rng,
    ));

    state.mode = Mode::Reward {
        reward_id_cards: Vec::new(),
        reward_id_relics: id_relics_reward,
        reward_id_potions: Vec::new(),
        reward_gold: gold,
    };
}

fn roll_gold_amount(rng: &mut impl Rng, chest_params: ChestParams) -> u16 {
    let base = chest_params.gold_base as f32;
    let factor = rng.random_range(0.9..=1.1);
    (base * factor).round() as u16
}
