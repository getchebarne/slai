mod abacus;
mod akabeko;
mod anchor;
mod ancient_tea_set;
mod art_of_war;
mod astrolabe;
mod bag_of_marbles;
mod bag_of_preparation;
mod bird_faced_urn;
mod black_star;
mod blood_vial;
mod blue_candle;
mod boot;
mod bottled_flame;
mod bottled_lightning;
mod bottled_tornado;
mod bronze_scales;
mod busted_crown;
mod calipers;
mod calling_bell;
mod captains_wheel;
mod cauldron;
mod centennial_puzzle;
mod ceramic_fish;
mod chemical_x;
mod circlet;
mod clockwork_souvenir;
mod coffee_dripper;
mod cultist_headpiece;
mod cursed_key;
mod darkstone_periapt;
mod dead_branch;
mod dollys_mirror;
mod dream_catcher;
mod du_vu_doll;
mod ectoplasm;
mod empty_cage;
mod eternal_feather;
mod face_of_cleric;
mod fossilized_helix;
mod frozen_egg;
mod fusion_hammer;
mod gambling_chip;
mod ginger;
mod girya;
mod golden_idol;
mod gremlin_horn;
mod gremlin_visage;
mod hand_drill;
mod happy_flower;
mod horn_cleat;
mod hovering_kite;
mod ice_cream;
mod incense_burner;
mod ink_bottle;
mod juzu_bracelet;
mod kunai;
mod lantern;
mod lees_waffle;
mod letter_opener;
mod lizard_tail;
mod mango;
mod matryoshka;
mod maw_bank;
mod meal_ticket;
mod meat_on_the_bone;
mod medical_kit;
mod membership_card;
mod mercury_hourglass;
mod molten_egg;
mod mummified_hand;
mod ninja_scroll;
mod nloths_hungry_face;
mod nunchaku;
mod odd_mushroom;
mod oddly_smooth_stone;
mod old_coin;
mod omamori;
mod orange_pellets;
mod orichalcum;
mod ornamental_fan;
mod orrery;
mod pandoras_box;
mod pantograph;
mod paper_krane;
mod peace_pipe;
mod pear;
mod pen_nib;
mod philosopher_stone;
mod pocketwatch;
mod potion_belt;
mod prayer_wheel;
mod preserved_insect;
mod question_card;
mod red_mask;
mod regal_pillow;
mod ring_of_the_serpent;
mod runic_pyramid;
mod sacred_bark;
mod shovel;
mod shuriken;
mod singing_bowl;
mod slavers_collar;
mod sling_of_courage;
mod smiling_mask;
mod snake_ring;
mod snecko_eye;
mod snecko_skull;
mod sozu;
mod spirit_poop;
mod ssserpent_head;
mod stone_calendar;
mod strange_spoon;
mod strawberry;
mod strike_dummy;
mod sundial;
mod the_courier;
mod the_specimen;
mod thread_and_needle;
mod tingsha;
mod tiny_chest;
mod tiny_house;
mod toolbox;
mod torii;
mod tough_bandages;
mod toxic_egg;
mod toy_ornithopter;
mod tungsten_rod;
mod turnip;
mod twisted_funnel;
mod unceasing_top;
mod vajra;
mod velvet_choker;
mod war_paint;
mod warped_tongs;
mod whetstone;
mod white_beast_statue;
mod wing_boots;
mod wrist_blade;

use strum::EnumCount;

use crate::entity::Entity;
use crate::types::CardKind;
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
        RelicName::MealTicket => meal_ticket::MEAL_TICKET,
        RelicName::MawBank => maw_bank::MAW_BANK,
        RelicName::JuzuBracelet => juzu_bracelet::JUZU_BRACELET,
        RelicName::TinyChest => tiny_chest::TINY_CHEST,
        RelicName::EternalFeather => eternal_feather::ETERNAL_FEATHER,
        RelicName::AncientTeaSet => ancient_tea_set::ANCIENT_TEA_SET,
        RelicName::RegalPillow => regal_pillow::REGAL_PILLOW,
        RelicName::MeatOnTheBone => meat_on_the_bone::MEAT_ON_THE_BONE,
        RelicName::Omamori => omamori::OMAMORI,
        RelicName::DarkstonePeriapt => darkstone_periapt::DARKSTONE_PERIAPT,
        RelicName::CeramicFish => ceramic_fish::CERAMIC_FISH,
        RelicName::FrozenEgg => frozen_egg::FROZEN_EGG,
        RelicName::MoltenEgg => molten_egg::MOLTEN_EGG,
        RelicName::ToxicEgg => toxic_egg::TOXIC_EGG,
        RelicName::ToyOrnithopter => toy_ornithopter::TOY_ORNITHOPTER,
        RelicName::SmilingMask => smiling_mask::SMILING_MASK,
        RelicName::DeadBranch => dead_branch::DEAD_BRANCH,
        RelicName::DuVuDoll => du_vu_doll::DU_VU_DOLL,
        RelicName::Pantograph => pantograph::PANTOGRAPH,
        RelicName::SlingOfCourage => sling_of_courage::SLING_OF_COURAGE,
        RelicName::Strawberry => strawberry::STRAWBERRY,
        RelicName::Pear => pear::PEAR,
        RelicName::Mango => mango::MANGO,
        RelicName::OldCoin => old_coin::OLD_COIN,
        RelicName::PotionBelt => potion_belt::POTION_BELT,
        RelicName::WarPaint => war_paint::WAR_PAINT,
        RelicName::Whetstone => whetstone::WHETSTONE,
        RelicName::EmptyCage => empty_cage::EMPTY_CAGE,
        RelicName::PandorasBox => pandoras_box::PANDORAS_BOX,
        RelicName::PenNib => pen_nib::PEN_NIB,
        RelicName::FossilizedHelix => fossilized_helix::FOSSILIZED_HELIX,
        RelicName::PreservedInsect => preserved_insect::PRESERVED_INSECT,
        RelicName::UnceasingTop => unceasing_top::UNCEASING_TOP,
        RelicName::BlueCandle => blue_candle::BLUE_CANDLE,
        RelicName::MedicalKit => medical_kit::MEDICAL_KIT,
        RelicName::SpiritPoop => spirit_poop::SPIRIT_POOP,
        RelicName::WarpedTongs => warped_tongs::WARPED_TONGS,
        RelicName::CultistHeadpiece => cultist_headpiece::CULTIST_HEADPIECE,
        RelicName::FaceOfCleric => face_of_cleric::FACE_OF_CLERIC,
        RelicName::NlothsHungryFace => nloths_hungry_face::NLOTHS_HUNGRY_FACE,
        RelicName::SsserpentHead => ssserpent_head::SSSERPENT_HEAD,
        RelicName::OddMushroom => odd_mushroom::ODD_MUSHROOM,
        RelicName::PhilosopherStone => philosopher_stone::PHILOSOPHER_STONE,
        RelicName::CoffeeDripper => coffee_dripper::COFFEE_DRIPPER,
        RelicName::FusionHammer => fusion_hammer::FUSION_HAMMER,
        RelicName::Sozu => sozu::SOZU,
        RelicName::CursedKey => cursed_key::CURSED_KEY,
        RelicName::BustedCrown => busted_crown::BUSTED_CROWN,
        RelicName::SlaversCollar => slavers_collar::SLAVERS_COLLAR,
        RelicName::Ectoplasm => ectoplasm::ECTOPLASM,
        RelicName::VelvetChoker => velvet_choker::VELVET_CHOKER,
        RelicName::WristBlade => wrist_blade::WRIST_BLADE,
        RelicName::HoveringKite => hovering_kite::HOVERING_KITE,
        RelicName::DreamCatcher => dream_catcher::DREAM_CATCHER,
        RelicName::Cauldron => cauldron::CAULDRON,
        RelicName::MembershipCard => membership_card::MEMBERSHIP_CARD,
        RelicName::TheCourier => the_courier::THE_COURIER,
        RelicName::GamblingChip => gambling_chip::GAMBLING_CHIP,
        RelicName::BottledFlame => bottled_flame::BOTTLED_FLAME,
        RelicName::BottledLightning => bottled_lightning::BOTTLED_LIGHTNING,
        RelicName::BottledTornado => bottled_tornado::BOTTLED_TORNADO,
        RelicName::Matryoshka => matryoshka::MATRYOSHKA,
        RelicName::Orrery => orrery::ORRERY,
        RelicName::Toolbox => toolbox::TOOLBOX,
        RelicName::SneckoEye => snecko_eye::SNECKO_EYE,
        RelicName::Astrolabe => astrolabe::ASTROLABE,
        RelicName::CallingBell => calling_bell::CALLING_BELL,
        RelicName::TinyHouse => tiny_house::TINY_HOUSE,
        RelicName::BlackStar => black_star::BLACK_STAR,
        RelicName::Girya => girya::GIRYA,
        RelicName::PeacePipe => peace_pipe::PEACE_PIPE,
        RelicName::Shovel => shovel::SHOVEL,
        RelicName::WingBoots => wing_boots::WING_BOOTS,
        RelicName::QuestionCard => question_card::QUESTION_CARD,
        RelicName::SingingBowl => singing_bowl::SINGING_BOWL,
        RelicName::PrayerWheel => prayer_wheel::PRAYER_WHEEL,
        RelicName::RunicPyramid => runic_pyramid::RUNIC_PYRAMID,
        RelicName::RingOfTheSerpent => ring_of_the_serpent::RING_OF_THE_SERPENT,
        RelicName::SacredBark => sacred_bark::SACRED_BARK,
    }
}

// Bump a Relic's counter if owned; at `threshold` reset it to 0 and report the fire
pub fn trigger_relic_counter(
    name: RelicName,
    threshold: i16,
    id_relics: &[Option<usize>; RelicName::COUNT],
    entities: &mut [Entity],
) -> bool {
    let Some(id) = id_relics[name as usize] else {
        // If the Relic is not owned, return
        return false;
    };

    // Increase counter
    let counter = &mut entities[id].relic_counter;
    *counter += 1;

    // Reset if needed
    if *counter >= threshold {
        *counter = 0;
        return true;
    }
    false
}

// Frozen / Molten / Toxic Egg: Cards of the matching kind are obtained upgraded
pub fn egg_upgrades_kind(kind: CardKind, id_relics: &[Option<usize>; RelicName::COUNT]) -> bool {
    let egg = match kind {
        CardKind::Power => RelicName::FrozenEgg,
        CardKind::Attack => RelicName::MoltenEgg,
        CardKind::Skill => RelicName::ToxicEgg,
        _ => return false,
    };
    id_relics[egg as usize].is_some()
}

// Per-turn Relic counters; reset at character turn end and at combat start
// (combat can end mid-turn, so turn-end resets alone leak into the next combat)
pub const RELIC_COUNTERS_PER_TURN: &[RelicName] = &[
    RelicName::Kunai,
    RelicName::Shuriken,
    RelicName::OrnamentalFan,
    RelicName::LetterOpener,
    RelicName::OrangePellets,
    RelicName::HoveringKite,
];

// Per-combat Relic counters; reset at combat start only
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
    &ancient_tea_set::ANCIENT_TEA_SET,
    &art_of_war::ART_OF_WAR,
    &bag_of_marbles::BAG_OF_MARBLES,
    &bag_of_preparation::BAG_OF_PREPARATION,
    &bird_faced_urn::BIRD_FACED_URN,
    &blood_vial::BLOOD_VIAL,
    &blue_candle::BLUE_CANDLE,
    &boot::BOOT,
    &bronze_scales::BRONZE_SCALES,
    &calipers::CALIPERS,
    &captains_wheel::CAPTAINS_WHEEL,
    &centennial_puzzle::CENTENNIAL_PUZZLE,
    &ceramic_fish::CERAMIC_FISH,
    &chemical_x::CHEMICAL_X,
    &circlet::CIRCLET,
    &clockwork_souvenir::CLOCKWORK_SOUVENIR,
    &darkstone_periapt::DARKSTONE_PERIAPT,
    &dead_branch::DEAD_BRANCH,
    &dollys_mirror::DOLLYS_MIRROR,
    &du_vu_doll::DU_VU_DOLL,
    &empty_cage::EMPTY_CAGE,
    &eternal_feather::ETERNAL_FEATHER,
    &fossilized_helix::FOSSILIZED_HELIX,
    &frozen_egg::FROZEN_EGG,
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
    &juzu_bracelet::JUZU_BRACELET,
    &kunai::KUNAI,
    &lantern::LANTERN,
    &lees_waffle::LEES_WAFFLE,
    &letter_opener::LETTER_OPENER,
    &lizard_tail::LIZARD_TAIL,
    &mango::MANGO,
    &maw_bank::MAW_BANK,
    &meal_ticket::MEAL_TICKET,
    &meat_on_the_bone::MEAT_ON_THE_BONE,
    &medical_kit::MEDICAL_KIT,
    &mercury_hourglass::MERCURY_HOURGLASS,
    &molten_egg::MOLTEN_EGG,
    &mummified_hand::MUMMIFIED_HAND,
    &ninja_scroll::NINJA_SCROLL,
    &nunchaku::NUNCHAKU,
    &oddly_smooth_stone::ODDLY_SMOOTH_STONE,
    &old_coin::OLD_COIN,
    &omamori::OMAMORI,
    &orange_pellets::ORANGE_PELLETS,
    &orichalcum::ORICHALCUM,
    &ornamental_fan::ORNAMENTAL_FAN,
    &pandoras_box::PANDORAS_BOX,
    &pantograph::PANTOGRAPH,
    &paper_krane::PAPER_KRANE,
    &pear::PEAR,
    &pen_nib::PEN_NIB,
    &pocketwatch::POCKETWATCH,
    &potion_belt::POTION_BELT,
    &preserved_insect::PRESERVED_INSECT,
    &red_mask::RED_MASK,
    &regal_pillow::REGAL_PILLOW,
    &shuriken::SHURIKEN,
    &sling_of_courage::SLING_OF_COURAGE,
    &smiling_mask::SMILING_MASK,
    &snecko_skull::SNECKO_SKULL,
    &stone_calendar::STONE_CALENDAR,
    &strange_spoon::STRANGE_SPOON,
    &strawberry::STRAWBERRY,
    &strike_dummy::STRIKE_DUMMY,
    &sundial::SUNDIAL,
    &the_specimen::THE_SPECIMEN,
    &thread_and_needle::THREAD_AND_NEEDLE,
    &tingsha::TINGSHA,
    &tiny_chest::TINY_CHEST,
    &torii::TORII,
    &tough_bandages::TOUGH_BANDAGES,
    &toxic_egg::TOXIC_EGG,
    &toy_ornithopter::TOY_ORNITHOPTER,
    &tungsten_rod::TUNGSTEN_ROD,
    &turnip::TURNIP,
    &twisted_funnel::TWISTED_FUNNEL,
    &unceasing_top::UNCEASING_TOP,
    &vajra::VAJRA,
    &war_paint::WAR_PAINT,
    &whetstone::WHETSTONE,
    &white_beast_statue::WHITE_BEAST_STATUE,
    &spirit_poop::SPIRIT_POOP,
    &warped_tongs::WARPED_TONGS,
    &cultist_headpiece::CULTIST_HEADPIECE,
    &face_of_cleric::FACE_OF_CLERIC,
    &nloths_hungry_face::NLOTHS_HUNGRY_FACE,
    &ssserpent_head::SSSERPENT_HEAD,
    &odd_mushroom::ODD_MUSHROOM,
    &philosopher_stone::PHILOSOPHER_STONE,
    &coffee_dripper::COFFEE_DRIPPER,
    &fusion_hammer::FUSION_HAMMER,
    &sozu::SOZU,
    &cursed_key::CURSED_KEY,
    &busted_crown::BUSTED_CROWN,
    &slavers_collar::SLAVERS_COLLAR,
    &ectoplasm::ECTOPLASM,
    &velvet_choker::VELVET_CHOKER,
    &wrist_blade::WRIST_BLADE,
    &hovering_kite::HOVERING_KITE,
    &dream_catcher::DREAM_CATCHER,
    &cauldron::CAULDRON,
    &membership_card::MEMBERSHIP_CARD,
    &the_courier::THE_COURIER,
    &gambling_chip::GAMBLING_CHIP,
    &bottled_flame::BOTTLED_FLAME,
    &bottled_lightning::BOTTLED_LIGHTNING,
    &bottled_tornado::BOTTLED_TORNADO,
    &matryoshka::MATRYOSHKA,
    &orrery::ORRERY,
    &toolbox::TOOLBOX,
    &snecko_eye::SNECKO_EYE,
    &astrolabe::ASTROLABE,
    &calling_bell::CALLING_BELL,
    &tiny_house::TINY_HOUSE,
    &black_star::BLACK_STAR,
    &girya::GIRYA,
    &peace_pipe::PEACE_PIPE,
    &shovel::SHOVEL,
    &wing_boots::WING_BOOTS,
    &question_card::QUESTION_CARD,
    &singing_bowl::SINGING_BOWL,
    &prayer_wheel::PRAYER_WHEEL,
    &runic_pyramid::RUNIC_PYRAMID,
    &ring_of_the_serpent::RING_OF_THE_SERPENT,
    &sacred_bark::SACRED_BARK,
];
// Assert all Relics are included without duplicates
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

// Get number of Potions per tier-pool
const NUM_COMMON: usize = count_pool(RelicTier::Common);
const NUM_UNCOMMON: usize = count_pool(RelicTier::Uncommon);
const NUM_RARE: usize = count_pool(RelicTier::Rare);
const NUM_SHOP: usize = count_pool(RelicTier::Shop);

// Compute tier-pools
pub const POOL_COMMON_RELIC: &[RelicName] = &build_pool::<NUM_COMMON>(RelicTier::Common);
pub const POOL_UNCOMMON_RELIC: &[RelicName] = &build_pool::<NUM_UNCOMMON>(RelicTier::Uncommon);
pub const POOL_RARE_RELIC: &[RelicName] = &build_pool::<NUM_RARE>(RelicTier::Rare);
pub const POOL_SHOP_RELIC: &[RelicName] = &build_pool::<NUM_SHOP>(RelicTier::Shop);
