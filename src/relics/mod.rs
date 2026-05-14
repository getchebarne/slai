// Relic registry and accessors. Storage is a name-indexed
// `[Option<usize>; RelicName::COUNT]` on GameState; per-relic state lives on
// the Entity (counter, used_up, effects-on-combat-start)

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
        .filter_map(|(i, &opt)| opt.map(|id| (RelicName::from_u8(i as u8), id)))
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

const fn relic_tier_eq(a: RelicTier, b: RelicTier) -> bool {
    matches!(
        (a, b),
        (RelicTier::Starter, RelicTier::Starter)
            | (RelicTier::Common, RelicTier::Common)
            | (RelicTier::Uncommon, RelicTier::Uncommon)
            | (RelicTier::Rare, RelicTier::Rare)
            | (RelicTier::Boss, RelicTier::Boss)
            | (RelicTier::Shop, RelicTier::Shop)
            | (RelicTier::Special, RelicTier::Special)
    )
}

const fn count_relic_pool(tier: RelicTier) -> usize {
    let mut n = 0;
    let mut i = 0;
    while i < ALL_RELICS.len() {
        if relic_tier_eq(ALL_RELICS[i].relic_tier, tier) {
            n += 1;
        }
        i += 1;
    }
    n
}

const fn build_relic_pool<const N: usize>(tier: RelicTier) -> [RelicName; N] {
    let mut buf = [RelicName::SnakeRing; N];
    let mut idx = 0;
    let mut i = 0;
    while i < ALL_RELICS.len() {
        let r = ALL_RELICS[i];
        if relic_tier_eq(r.relic_tier, tier) {
            buf[idx] = r.relic_name;
            idx += 1;
        }
        i += 1;
    }
    buf
}

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

const COMMON_RELIC_N: usize = count_relic_pool(RelicTier::Common);
const UNCOMMON_RELIC_N: usize = count_relic_pool(RelicTier::Uncommon);
const RARE_RELIC_N: usize = count_relic_pool(RelicTier::Rare);
const SHOP_RELIC_N: usize = count_relic_pool(RelicTier::Shop);
const BOSS_RELIC_N: usize = count_relic_pool(RelicTier::Boss);

pub const RELIC_POOL_COMMON: &[RelicName] = &build_relic_pool::<COMMON_RELIC_N>(RelicTier::Common);
pub const RELIC_POOL_UNCOMMON: &[RelicName] =
    &build_relic_pool::<UNCOMMON_RELIC_N>(RelicTier::Uncommon);
pub const RELIC_POOL_RARE: &[RelicName] = &build_relic_pool::<RARE_RELIC_N>(RelicTier::Rare);
#[allow(dead_code)]
pub const RELIC_POOL_SHOP: &[RelicName] = &build_relic_pool::<SHOP_RELIC_N>(RelicTier::Shop);
#[allow(dead_code)]
pub const RELIC_POOL_BOSS: &[RelicName] = &build_relic_pool::<BOSS_RELIC_N>(RelicTier::Boss);
