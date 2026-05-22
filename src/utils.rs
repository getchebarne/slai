use rand::Rng;
use strum::EnumCount;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::cards::get_card;
use crate::consts::CARD_REWARD_ROLL_CHANCE_RARE;
use crate::consts::CARD_REWARD_ROLL_CHANCE_UNCOMMON;
use crate::consts::CARD_REWARD_ROLL_OFFSET_BASE;
use crate::consts::CARD_REWARD_ROLL_OFFSET_MIN;
use crate::consts::FACTOR_VULN;
use crate::consts::FACTOR_WEAK;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::consts::MAX_MONSTERS;
use crate::engine::entities_push;
use crate::entity::Entity;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::ActiveContext;
use crate::types::CardName;
use crate::types::RelicName;

pub fn shuffle<T>(slice: &mut [T], rng: &mut impl Rng) {
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        slice.swap(i, j);
    }
}

pub fn reshuffle_discard_into_draw(
    id_pile_draw: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    rng: &mut impl Rng,
) {
    id_pile_draw.append(id_pile_discard);
    shuffle(&mut id_pile_draw[..], rng);
}

// Fills `buf` with the ids of monsters that are alive, returns how many
// Callers use `&buf[..n]` as a slice. Zero heap allocation. Returns 0 when
// not in Combat context
pub fn fill_alive_monster_ids(state: &GameState, buf_alive: &mut [usize; MAX_MONSTERS]) -> usize {
    if !matches!(state.active, ActiveContext::Combat) {
        return 0;
    }
    let mut n = 0;
    for slot in state.id_monsters.iter() {
        if let Some(id) = *slot {
            buf_alive[n] = id;
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

    let id = entities_push(entities, get_relic(name));
    id
}

pub fn pick_from_pool(
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

// Roll MAX_COMBAT_CARD_REWARD distinct cards; pity-bumps reward_roll_offset toward rares
pub fn roll_card_rewards(
    id_character: usize,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
    out: &mut Vec<usize>,
) {
    let mut character_reward_roll_offset = entities[id_character].character_reward_roll_offset;
    let mut rolled_card_names: [CardName; MAX_COMBAT_CARD_REWARD] =
        [CardName::Strike; MAX_COMBAT_CARD_REWARD];

    out.clear();
    for _ in 0..MAX_COMBAT_CARD_REWARD {
        let roll = rng.random_range(0i32..99) + character_reward_roll_offset as i32;

        let pool = if roll < CARD_REWARD_ROLL_CHANCE_RARE {
            character_reward_roll_offset = CARD_REWARD_ROLL_OFFSET_BASE;
            POOL_RARE_GREEN_CARD
        } else if roll < CARD_REWARD_ROLL_CHANCE_UNCOMMON {
            POOL_UNCOMMON_GREEN_CARD
        } else {
            character_reward_roll_offset =
                (character_reward_roll_offset - 1).max(CARD_REWARD_ROLL_OFFSET_MIN);
            POOL_COMMON_GREEN_CARD
        };

        let mut name = pool[rng.random_range(0..pool.len())];
        while rolled_card_names[..out.len()].contains(&name) {
            name = pool[rng.random_range(0..pool.len())];
        }

        rolled_card_names[out.len()] = name;
        let card = get_card(name, false);
        let id_card = entities_push(entities, card);
        out.push(id_card);
    }

    entities[id_character].character_reward_roll_offset = character_reward_roll_offset;
}
