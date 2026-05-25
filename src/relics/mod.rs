mod akabeko;
mod anchor;
mod bag_of_marbles;
mod bag_of_preparation;
mod blood_vial;
mod bronze_scales;
mod circlet;
mod kunai;
mod ninja_scroll;
mod oddly_smooth_stone;
mod shuriken;
mod snake_ring;
mod thread_and_needle;
mod twisted_funnel;
mod vajra;

use strum::EnumCount;

use crate::entity::Entity;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::relic_name_from_u8;

pub fn get_relic(name: RelicName) -> Entity {
    match name {
        RelicName::SnakeRing => snake_ring::SNAKE_RING,
        RelicName::Akabeko => akabeko::AKABEKO,
        RelicName::Anchor => anchor::ANCHOR,
        RelicName::BagOfMarbles => bag_of_marbles::BAG_OF_MARBLES,
        RelicName::BagOfPreparation => bag_of_preparation::BAG_OF_PREPARATION,
        RelicName::BloodVial => blood_vial::BLOOD_VIAL,
        RelicName::BronzeScales => bronze_scales::BRONZE_SCALES,
        RelicName::Kunai => kunai::KUNAI,
        RelicName::NinjaScroll => ninja_scroll::NINJA_SCROLL,
        RelicName::OddlySmoothStone => oddly_smooth_stone::ODDLY_SMOOTH_STONE,
        RelicName::Shuriken => shuriken::SHURIKEN,
        RelicName::ThreadAndNeedle => thread_and_needle::THREAD_AND_NEEDLE,
        RelicName::TwistedFunnel => twisted_funnel::TWISTED_FUNNEL,
        RelicName::Vajra => vajra::VAJRA,
        RelicName::Circlet => circlet::CIRCLET,
    }
}

pub fn iter_owned_relics(
    id_relics: &[Option<usize>; RelicName::COUNT],
) -> impl Iterator<Item = (RelicName, usize)> + '_ {
    id_relics
        .iter()
        .enumerate()
        .filter_map(|(i, &opt)| opt.map(|id| (relic_name_from_u8(i as u8), id)))
}

pub const ALL_RELICS: &[&'static Entity] = &[
    &snake_ring::SNAKE_RING,
    &akabeko::AKABEKO,
    &anchor::ANCHOR,
    &bag_of_marbles::BAG_OF_MARBLES,
    &bag_of_preparation::BAG_OF_PREPARATION,
    &blood_vial::BLOOD_VIAL,
    &bronze_scales::BRONZE_SCALES,
    &circlet::CIRCLET,
    &kunai::KUNAI,
    &ninja_scroll::NINJA_SCROLL,
    &oddly_smooth_stone::ODDLY_SMOOTH_STONE,
    &shuriken::SHURIKEN,
    &thread_and_needle::THREAD_AND_NEEDLE,
    &twisted_funnel::TWISTED_FUNNEL,
    &vajra::VAJRA,
];
// Assert all relics are included without duplicates
const _: () = assert!(ALL_RELICS.len() == RelicName::COUNT);
const _: () = {
    let mut seen = [false; RelicName::COUNT];
    let mut i = 0;
    while i < ALL_RELICS.len() {
        let idx = ALL_RELICS[i].relic_name as usize;
        assert!(!seen[idx], "ALL_RELICS contains a duplicate RelicName");
        seen[idx] = true;
        i += 1;
    }
};

const fn relic_tier_eq(lhs: RelicTier, rhs: RelicTier) -> bool {
    matches!(
        (lhs, rhs),
        (RelicTier::Starter, RelicTier::Starter)
            | (RelicTier::Common, RelicTier::Common)
            | (RelicTier::Uncommon, RelicTier::Uncommon)
            | (RelicTier::Rare, RelicTier::Rare)
            | (RelicTier::Boss, RelicTier::Boss)
            | (RelicTier::Shop, RelicTier::Shop)
            | (RelicTier::Special, RelicTier::Special)
    )
}

const fn count_pool(tier: RelicTier) -> usize {
    let mut count = 0;
    let mut idx = 0;
    while idx < ALL_RELICS.len() {
        if relic_tier_eq(ALL_RELICS[idx].relic_tier, tier) {
            count += 1;
        }
        idx += 1;
    }
    count
}

const fn build_pool<const N: usize>(tier: RelicTier) -> [RelicName; N] {
    let mut buf = [RelicName::SnakeRing; N];
    let mut idx_pool = 0;
    let mut idx_all = 0;
    while idx_all < ALL_RELICS.len() {
        let relic = ALL_RELICS[idx_all];
        if relic_tier_eq(relic.relic_tier, tier) {
            buf[idx_pool] = relic.relic_name;
            idx_pool += 1;
        }
        idx_all += 1;
    }
    buf
}

// Get number of potions per tier-pool
const NUM_COMMON: usize = count_pool(RelicTier::Common);
const NUM_UNCOMMON: usize = count_pool(RelicTier::Uncommon);
const NUM_RARE: usize = count_pool(RelicTier::Rare);
const NUM_SHOP: usize = count_pool(RelicTier::Shop);
const NUM_BOSS: usize = count_pool(RelicTier::Boss);

// Compute tier-pools
pub const POOL_COMMON_RELIC: &[RelicName] = &build_pool::<NUM_COMMON>(RelicTier::Common);
pub const POOL_UNCOMMON_RELIC: &[RelicName] = &build_pool::<NUM_UNCOMMON>(RelicTier::Uncommon);
pub const POOL_RARE_RELIC: &[RelicName] = &build_pool::<NUM_RARE>(RelicTier::Rare);
#[allow(dead_code)]
pub const POOL_SHOP_RELIC: &[RelicName] = &build_pool::<NUM_SHOP>(RelicTier::Shop);
#[allow(dead_code)]
pub const POOL_BOSS_RELIC: &[RelicName] = &build_pool::<NUM_BOSS>(RelicTier::Boss);
