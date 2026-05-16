use rand::Rng;
use strum::EnumCount;

use crate::cards::POOL_COMMON_CARD;
use crate::cards::POOL_RARE_CARD;
use crate::cards::POOL_UNCOMMON_CARD;
use crate::cards::get_card;
use crate::consts::CARD_REWARD_ROLL_OFFSET_BASE;
use crate::consts::CARD_REWARD_ROLL_OFFSET_MIN;
use crate::consts::CHANCE_RARE;
use crate::consts::CHANCE_UNCOMMON;
use crate::consts::FACTOR_VULN;
use crate::consts::FACTOR_WEAK;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::consts::MAX_MONSTERS;
use crate::entity::Entity;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::CardName;
use crate::types::RelicName;

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

// Fills `buf` with the ids of monsters that are alive, returns how many
// Callers use `&buf[..n]` as a slice. Zero heap allocation
pub fn fill_alive_monster_ids(state: &GameState, buf_alive: &mut [usize; MAX_MONSTERS]) -> usize {
    let mut n = 0;
    for i in 0..state.monster_count as usize {
        let id_monster = state.id_monsters[i];
        if !state.entities[id_monster].dead {
            buf_alive[n] = id_monster;
            n += 1;
        }
    }
    n
}

// Strength, Weak, and Vulnerable scaling shared between the live damage pipeline
// and the FFI intent view
pub fn scale_attack_damage(
    base: u16,
    source_str_stacks: i16,
    source_is_weak: bool,
    target_is_vulnerable: bool,
) -> u16 {
    let mut value = base as f32 + source_str_stacks as f32;
    if source_is_weak {
        value *= FACTOR_WEAK;
    }
    if target_is_vulnerable {
        value *= FACTOR_VULN;
    }
    value.max(0.0) as u16
}

pub fn remove_card_from_collection(id_target: usize, id_collection: &mut Vec<usize>) {
    let pos = id_collection
        .iter()
        .position(|&elem| elem == id_target)
        .expect("Can't remove a card that's not in the collection");

    id_collection.remove(pos);
}

use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;

// Used by both elite combat-end and chest opening
pub fn add_relic_reward_for_roll(
    roll: u8,
    th_common: u8,
    th_uncommon: u8,
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> usize {
    let name = if roll < th_common {
        pick_from_pool(POOL_COMMON_RELIC, id_relics, rng)
            .or_else(|| pick_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng))
            .or_else(|| pick_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet)
    } else if roll < th_uncommon {
        pick_from_pool(POOL_UNCOMMON_RELIC, id_relics, rng)
            .or_else(|| pick_from_pool(POOL_RARE_RELIC, id_relics, rng))
            .unwrap_or(RelicName::Circlet)
    } else {
        pick_from_pool(POOL_RARE_RELIC, id_relics, rng).unwrap_or(RelicName::Circlet)
    };

    let id = entities.len();
    entities.push(get_relic(name));
    id
}

fn pick_from_pool(
    pool: &[RelicName],
    id_relics: &[Option<usize>; RelicName::COUNT],
    rng: &mut impl Rng,
) -> Option<RelicName> {
    let mut candidates = [RelicName::SnakeRing; RelicName::COUNT];
    let mut n = 0;
    for &name in pool {
        if id_relics[name as usize].is_none() {
            candidates[n] = name;
            n += 1;
        }
    }
    if n == 0 {
        None
    } else {
        Some(candidates[rng.random_range(0..n)])
    }
}

// Roll MAX_COMBAT_CARD_REWARD distinct cards. Mutates character_reward_roll_offset
// (pity bias toward rares) and returns the spawned entity ids
pub fn roll_card_rewards(
    id_character: usize,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> Vec<usize> {
    let mut character_reward_roll_offset = entities[id_character].character_reward_roll_offset;
    let mut rolled_card_names: Vec<CardName> = Vec::new();
    let mut rolled_ids: Vec<usize> = Vec::with_capacity(MAX_COMBAT_CARD_REWARD);

    for _ in 0..MAX_COMBAT_CARD_REWARD {
        let roll = rng.random_range(0i32..99) + character_reward_roll_offset as i32;

        let pool = if roll < CHANCE_RARE {
            character_reward_roll_offset = CARD_REWARD_ROLL_OFFSET_BASE;
            POOL_RARE_CARD
        } else if roll < CHANCE_UNCOMMON {
            POOL_UNCOMMON_CARD
        } else {
            character_reward_roll_offset =
                (character_reward_roll_offset - 1).max(CARD_REWARD_ROLL_OFFSET_MIN);
            POOL_COMMON_CARD
        };

        let mut name = pool[rng.random_range(0..pool.len())];
        while rolled_card_names.contains(&name) {
            name = pool[rng.random_range(0..pool.len())];
        }

        rolled_card_names.push(name);
        let card = get_card(name, false); // TODO: can generate upgraded cards on Act2+
        let id_card = entities.len();
        entities.push(card);
        rolled_ids.push(id_card);
    }

    entities[id_character].character_reward_roll_offset = character_reward_roll_offset;
    rolled_ids
}
