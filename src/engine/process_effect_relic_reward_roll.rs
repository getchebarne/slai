use rand::Rng;
use strum::EnumCount;

use crate::consts::TierWeights;
use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::relics::get_relic;
use crate::types::RelicName;

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

pub fn process_effect_relic_reward_roll(
    weights: TierWeights,
    id_relics: &[Option<usize>; RelicName::COUNT],
    id_relic_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    let roll = rng.random_range(0..100) as u8;
    add_relic_reward_for_roll(roll, weights, id_relics, id_relic_rewards, entities, rng);
    DispatchResult::Continue
}

// Shared tier-dispatch + pool-pick used by both `RelicRewardRoll` (elite) and
// `ChestOpen` (which must reuse a single roll for gold-yes/no AND relic-tier)
pub fn add_relic_reward_for_roll(
    roll: u8,
    weights: TierWeights,
    id_relics: &[Option<usize>; RelicName::COUNT],
    id_relic_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) {
    let pool: &[RelicName] = if roll < weights.common_threshold {
        RELIC_POOL_COMMON
    } else if roll < weights.uncommon_threshold {
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
