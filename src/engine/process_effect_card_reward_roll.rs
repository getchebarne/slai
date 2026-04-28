use rand::Rng;

use crate::cards::{REWARD_POOL_COMMON, REWARD_POOL_RARE, REWARD_POOL_UNCOMMON, get_card};
use crate::consts::{
    CARD_REWARD_ROLL_OFFSET_BASE, CARD_REWARD_ROLL_OFFSET_MIN, CHANCE_RARE, CHANCE_UNCOMMON,
    MAX_COMBAT_CARD_REWARD,
};
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::CardName;

pub fn process_effect_card_reward_roll(
    id_character: usize,
    id_card_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    let mut reward_roll_offset = entities[id_character].reward_roll_offset;
    let mut rolled_card_names: Vec<CardName> = Vec::new();

    for _ in 0..MAX_COMBAT_CARD_REWARD {
        // Roll rarity w/ pity offset
        let roll = rng.random_range(0i32..99) + reward_roll_offset as i32;

        // Select pool and update offset
        let pool = if roll < CHANCE_RARE {
            reward_roll_offset = CARD_REWARD_ROLL_OFFSET_BASE;
            REWARD_POOL_RARE
        } else if roll < CHANCE_UNCOMMON {
            REWARD_POOL_UNCOMMON
        } else {
            reward_roll_offset = (reward_roll_offset - 1).max(CARD_REWARD_ROLL_OFFSET_MIN);
            REWARD_POOL_COMMON
        };

        // Pick a card name, re-roll on duplicates
        let mut name = pool[rng.random_range(0..pool.len())];
        while rolled_card_names.contains(&name) {
            name = pool[rng.random_range(0..pool.len())];
        }

        rolled_card_names.push(name);
        let card = get_card(name, false); // TODO: can generate upgraded cards on Act2+
        let id_card = entities.len();
        entities.push(card);
        id_card_rewards.push(id_card);
    }

    entities[id_character].reward_roll_offset = reward_roll_offset;

    // No halt push: the queue drains and the engine derives
    // Phase::CombatReward from non-empty `id_card_rewards`.
    DispatchResult::Continue
}
