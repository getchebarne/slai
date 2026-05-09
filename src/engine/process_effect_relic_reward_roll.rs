use rand::Rng;

use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::relics::{get_relic, has_relic};
use crate::types::{N_RELICS, RelicName};

const RELIC_POOL_COMMON: &[RelicName] = &[
    RelicName::Akabeko,
    RelicName::Anchor,
    RelicName::BagOfMarbles,
    RelicName::BagOfPreparation,
    RelicName::BloodVial,
    RelicName::BronzeScales,
    RelicName::OddlySmoothStone,
    RelicName::ThreadAndNeedle,
    RelicName::Vajra,
];

const RELIC_POOL_UNCOMMON: &[RelicName] = &[
    RelicName::Kunai,
    RelicName::NinjaScroll,
    RelicName::TwistedFunnel,
];

const RELIC_POOL_RARE: &[RelicName] = &[RelicName::Shuriken];

pub fn process_effect_relic_reward_roll(
    relics_active: u128,
    id_relic_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    // 60/30/10 common/uncommon/rare. Java is 50/33/17; tune later
    let roll = rng.random_range(0..100);
    let pool: &[RelicName] = if roll < 60 {
        RELIC_POOL_COMMON
    } else if roll < 90 {
        RELIC_POOL_UNCOMMON
    } else {
        RELIC_POOL_RARE
    };

    let pick = pick_from_pool(pool, relics_active, rng)
        .or_else(|| pick_from_pool(RELIC_POOL_RARE, relics_active, rng))
        .or_else(|| pick_from_pool(RELIC_POOL_UNCOMMON, relics_active, rng))
        .or_else(|| pick_from_pool(RELIC_POOL_COMMON, relics_active, rng));

    if let Some(name) = pick {
        let id = entities.len();
        entities.push(get_relic(name));
        id_relic_rewards.push(id);
    }
    DispatchResult::Continue
}

fn pick_from_pool(
    pool: &[RelicName],
    relics_active: u128,
    rng: &mut impl Rng,
) -> Option<RelicName> {
    let mut candidates = [RelicName::SnakeRing; N_RELICS];
    let mut n = 0;
    for &name in pool {
        if !has_relic(relics_active, name) {
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
