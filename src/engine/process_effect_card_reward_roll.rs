use rand::Rng;

use crate::cards::{REWARD_POOL_COMMON, REWARD_POOL_RARE, REWARD_POOL_UNCOMMON, get_card};
use crate::consts::{
    CARD_REWARD_ROLL_OFFSET_BASE, CARD_REWARD_ROLL_OFFSET_MIN, CHANCE_RARE, CHANCE_UNCOMMON,
    MAX_COMBAT_CARD_REWARD,
};
use crate::engine::ProcessEffectResult;
use crate::state::{Entity, EntityKind};
use crate::types::{CardName, EntityId, Phase};

pub fn process_effect_card_reward_roll(
    character: EntityId,
    card_rewards: &mut Vec<EntityId>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> ProcessEffectResult {
    // Read current roll offset
    let mut reward_roll_offset = entities[character.0 as usize]
        .kind
        .character_ref()
        .reward_roll_offset;
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

        // Create the card entity and add to rewards
        rolled_card_names.push(name);
        let card = get_card(name, false); // TODO: can generate upgraded cards on Act2+
        let id = EntityId(entities.len() as u32);
        entities.push(Entity {
            kind: EntityKind::Card(card),
        });
        card_rewards.push(id);
    }

    // Persist updated roll offset
    entities[character.0 as usize]
        .kind
        .character_mut()
        .reward_roll_offset = reward_roll_offset;

    // Halt and wait for player's combat reward selection
    ProcessEffectResult::Halt {
        phase_new: Phase::CombatReward,
    }
}
