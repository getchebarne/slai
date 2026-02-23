use rand::Rng;

use crate::cards::{Card, REWARD_POOL_COMMON, REWARD_POOL_RARE, REWARD_POOL_UNCOMMON, get_card};
use crate::consts::{
    CARD_REWARD_ROLL_OFFSET_BASE, CARD_REWARD_ROLL_OFFSET_MIN, CHANCE_RARE, CHANCE_UNCOMMON,
    MAX_COMBAT_CARD_REWARD,
};
use crate::effect::Effect;
use crate::engine::ProcessEffectResult;
use crate::types::CardName;

pub fn process_effect_card_reward_roll(
    card_rewards: &mut Vec<Card>,
    reward_roll_offset: &mut i8,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    card_rewards.clear();
    let mut rolled_card_names: Vec<CardName> = Vec::new();

    for _ in 0..MAX_COMBAT_CARD_REWARD {
        let roll = rng.random_range(0i32..99) + *reward_roll_offset as i32;

        // Sample reward pool
        let pool = if roll < CHANCE_RARE {
            *reward_roll_offset = CARD_REWARD_ROLL_OFFSET_BASE;
            REWARD_POOL_RARE
        } else if roll < CHANCE_UNCOMMON {
            REWARD_POOL_UNCOMMON
        } else {
            *reward_roll_offset = (*reward_roll_offset - 1).max(CARD_REWARD_ROLL_OFFSET_MIN);
            REWARD_POOL_COMMON
        };

        // Sample from reward pool
        let mut name = pool[rng.random_range(0..pool.len())];
        while rolled_card_names.contains(&name) {
            name = pool[rng.random_range(0..pool.len())];
        }

        // Append
        rolled_card_names.push(name);
        card_rewards.push(get_card(name, false));
    }

    ProcessEffectResult::Continue {
        top: Vec::new(),
        bot: vec![Effect::AwaitCardReward],
    }
}
