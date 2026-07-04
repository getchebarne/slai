mod abacus;
mod akabeko;
mod anchor;
mod art_of_war;
mod bag_of_marbles;
mod bag_of_preparation;
mod bird_faced_urn;
mod blood_vial;
mod boot;
mod bronze_scales;
mod calipers;
mod captains_wheel;
mod centennial_puzzle;
mod chemical_x;
mod circlet;
mod clockwork_souvenir;
mod dollys_mirror;
mod ginger;
mod golden_idol;
mod gremlin_horn;
mod gremlin_visage;
mod hand_drill;
mod happy_flower;
mod horn_cleat;
mod ice_cream;
mod incense_burner;
mod ink_bottle;
mod kunai;
mod lantern;
mod lees_waffle;
mod letter_opener;
mod lizard_tail;
mod mercury_hourglass;
mod mummified_hand;
mod ninja_scroll;
mod nunchaku;
mod oddly_smooth_stone;
mod orange_pellets;
mod orichalcum;
mod ornamental_fan;
mod paper_krane;
mod pocketwatch;
mod red_mask;
mod shuriken;
mod snecko_skull;
mod stone_calendar;
mod strange_spoon;
mod strike_dummy;
mod sundial;
mod the_specimen;
mod tingsha;
mod torii;
mod tough_bandages;
mod tungsten_rod;
mod turnip;
mod white_beast_statue;
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
        RelicName::GoldenIdol => golden_idol::GOLDEN_IDOL,
        RelicName::Lantern => lantern::LANTERN,
        RelicName::ClockworkSouvenir => clockwork_souvenir::CLOCKWORK_SOUVENIR,
        RelicName::GremlinVisage => gremlin_visage::GREMLIN_VISAGE,
        RelicName::RedMask => red_mask::RED_MASK,
        RelicName::Nunchaku => nunchaku::NUNCHAKU,
        RelicName::InkBottle => ink_bottle::INK_BOTTLE,
        RelicName::LetterOpener => letter_opener::LETTER_OPENER,
        RelicName::OrnamentalFan => ornamental_fan::ORNAMENTAL_FAN,
        RelicName::BirdFacedUrn => bird_faced_urn::BIRD_FACED_URN,
        RelicName::MummifiedHand => mummified_hand::MUMMIFIED_HAND,
        RelicName::OrangePellets => orange_pellets::ORANGE_PELLETS,
        RelicName::StrangeSpoon => strange_spoon::STRANGE_SPOON,
        RelicName::ChemicalX => chemical_x::CHEMICAL_X,
        RelicName::ArtOfWar => art_of_war::ART_OF_WAR,
        RelicName::Orichalcum => orichalcum::ORICHALCUM,
        RelicName::Pocketwatch => pocketwatch::POCKETWATCH,
        RelicName::StoneCalendar => stone_calendar::STONE_CALENDAR,
        RelicName::Abacus => abacus::ABACUS,
        RelicName::Sundial => sundial::SUNDIAL,
        RelicName::WhiteBeastStatue => white_beast_statue::WHITE_BEAST_STATUE,
        RelicName::DollysMirror => dollys_mirror::DOLLYS_MIRROR,
        RelicName::LeesWaffle => lees_waffle::LEES_WAFFLE,
        RelicName::HappyFlower => happy_flower::HAPPY_FLOWER,
        RelicName::IncenseBurner => incense_burner::INCENSE_BURNER,
        RelicName::MercuryHourglass => mercury_hourglass::MERCURY_HOURGLASS,
        RelicName::HornCleat => horn_cleat::HORN_CLEAT,
        RelicName::CaptainsWheel => captains_wheel::CAPTAINS_WHEEL,
        RelicName::Calipers => calipers::CALIPERS,
        RelicName::IceCream => ice_cream::ICE_CREAM,
        RelicName::SneckoSkull => snecko_skull::SNECKO_SKULL,
        RelicName::Ginger => ginger::GINGER,
        RelicName::Turnip => turnip::TURNIP,
        RelicName::Tingsha => tingsha::TINGSHA,
        RelicName::ToughBandages => tough_bandages::TOUGH_BANDAGES,
        RelicName::GremlinHorn => gremlin_horn::GREMLIN_HORN,
        RelicName::TheSpecimen => the_specimen::THE_SPECIMEN,
        RelicName::LizardTail => lizard_tail::LIZARD_TAIL,
        RelicName::Boot => boot::BOOT,
        RelicName::Torii => torii::TORII,
        RelicName::TungstenRod => tungsten_rod::TUNGSTEN_ROD,
        RelicName::HandDrill => hand_drill::HAND_DRILL,
        RelicName::StrikeDummy => strike_dummy::STRIKE_DUMMY,
        RelicName::PaperKrane => paper_krane::PAPER_KRANE,
        RelicName::CentennialPuzzle => centennial_puzzle::CENTENNIAL_PUZZLE,
    }
}

// Bump a relic's counter if owned; at `threshold` reset it to 0 and report the fire
pub fn relic_counter_fire(
    name: RelicName,
    threshold: i16,
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &mut [Entity],
) -> bool {
    let Some(id) = id_relics[name as usize] else {
        return false;
    };
    let counter = &mut entities[id].relic_counter;
    *counter += 1;
    if *counter >= threshold {
        *counter = 0;
        return true;
    }
    false
}

// Per-turn relic counters; reset at character turn end and at combat start
// (combat can end mid-turn, so turn-end resets alone leak into the next combat)
pub const RELIC_COUNTERS_PER_TURN: &[RelicName] = &[
    RelicName::Kunai,
    RelicName::Shuriken,
    RelicName::OrnamentalFan,
    RelicName::LetterOpener,
    RelicName::OrangePellets,
];

// Per-combat relic counters; reset at combat start only
pub const RELIC_COUNTERS_PER_COMBAT: &[RelicName] = &[
    RelicName::StoneCalendar,
    RelicName::HornCleat,
    RelicName::CaptainsWheel,
    RelicName::CentennialPuzzle,
];

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
    &abacus::ABACUS,
    &akabeko::AKABEKO,
    &anchor::ANCHOR,
    &art_of_war::ART_OF_WAR,
    &bag_of_marbles::BAG_OF_MARBLES,
    &bag_of_preparation::BAG_OF_PREPARATION,
    &bird_faced_urn::BIRD_FACED_URN,
    &blood_vial::BLOOD_VIAL,
    &boot::BOOT,
    &bronze_scales::BRONZE_SCALES,
    &calipers::CALIPERS,
    &captains_wheel::CAPTAINS_WHEEL,
    &centennial_puzzle::CENTENNIAL_PUZZLE,
    &chemical_x::CHEMICAL_X,
    &circlet::CIRCLET,
    &clockwork_souvenir::CLOCKWORK_SOUVENIR,
    &dollys_mirror::DOLLYS_MIRROR,
    &ginger::GINGER,
    &golden_idol::GOLDEN_IDOL,
    &gremlin_horn::GREMLIN_HORN,
    &gremlin_visage::GREMLIN_VISAGE,
    &hand_drill::HAND_DRILL,
    &happy_flower::HAPPY_FLOWER,
    &horn_cleat::HORN_CLEAT,
    &ice_cream::ICE_CREAM,
    &incense_burner::INCENSE_BURNER,
    &ink_bottle::INK_BOTTLE,
    &kunai::KUNAI,
    &lantern::LANTERN,
    &lees_waffle::LEES_WAFFLE,
    &letter_opener::LETTER_OPENER,
    &lizard_tail::LIZARD_TAIL,
    &mercury_hourglass::MERCURY_HOURGLASS,
    &mummified_hand::MUMMIFIED_HAND,
    &ninja_scroll::NINJA_SCROLL,
    &nunchaku::NUNCHAKU,
    &oddly_smooth_stone::ODDLY_SMOOTH_STONE,
    &orange_pellets::ORANGE_PELLETS,
    &orichalcum::ORICHALCUM,
    &ornamental_fan::ORNAMENTAL_FAN,
    &paper_krane::PAPER_KRANE,
    &pocketwatch::POCKETWATCH,
    &red_mask::RED_MASK,
    &shuriken::SHURIKEN,
    &snecko_skull::SNECKO_SKULL,
    &stone_calendar::STONE_CALENDAR,
    &strange_spoon::STRANGE_SPOON,
    &strike_dummy::STRIKE_DUMMY,
    &sundial::SUNDIAL,
    &the_specimen::THE_SPECIMEN,
    &thread_and_needle::THREAD_AND_NEEDLE,
    &tingsha::TINGSHA,
    &torii::TORII,
    &tough_bandages::TOUGH_BANDAGES,
    &tungsten_rod::TUNGSTEN_ROD,
    &turnip::TURNIP,
    &twisted_funnel::TWISTED_FUNNEL,
    &vajra::VAJRA,
    &white_beast_statue::WHITE_BEAST_STATUE,
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
