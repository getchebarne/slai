use rand::Rng;
use strum::EnumCount;

use crate::consts::FACTOR_VULN;
use crate::consts::FACTOR_WEAK;
use crate::consts::MAX_MONSTERS;
use crate::entity::Entity;
use crate::game::GameState;
use crate::relics::get_relic;
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

const RELIC_POOL_COMMON: &[RelicName] = &[
    RelicName::Akabeko,
    RelicName::Anchor,
    RelicName::BagOfMarbles,
    RelicName::BagOfPreparation,
    RelicName::BloodVial,
    RelicName::BronzeScales,
    RelicName::OddlySmoothStone,
    RelicName::Vajra,
];

const RELIC_POOL_UNCOMMON: &[RelicName] = &[
    RelicName::Kunai,
    RelicName::NinjaScroll,
    RelicName::Shuriken,
];

const RELIC_POOL_RARE: &[RelicName] = &[RelicName::ThreadAndNeedle];

// TwistedFunnel is shop-only — not offered by elites
#[allow(dead_code)]
const RELIC_POOL_SHOP: &[RelicName] = &[RelicName::TwistedFunnel];

// Used by both `EffectKind::RelicRewardRoll` (elite) and `EffectKind::ChestOpen`
pub fn add_relic_reward_for_roll(
    roll: u8,
    th_common: u8,
    th_uncommon: u8,
    id_relics: &[Option<usize>; RelicName::COUNT],
    id_relic_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) {
    let pool: &[RelicName] = if roll < th_common {
        RELIC_POOL_COMMON
    } else if roll < th_uncommon {
        RELIC_POOL_UNCOMMON
    } else {
        RELIC_POOL_RARE
    };

    let name = pick_from_pool(pool, id_relics, rng)
        .or_else(|| pick_from_pool(RELIC_POOL_RARE, id_relics, rng))
        .or_else(|| pick_from_pool(RELIC_POOL_UNCOMMON, id_relics, rng))
        .or_else(|| pick_from_pool(RELIC_POOL_COMMON, id_relics, rng))
        .unwrap_or(RelicName::Circlet);

    let id = entities.len();
    entities.push(get_relic(name));
    id_relic_rewards.push(id);
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
