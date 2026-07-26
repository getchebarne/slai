use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::action::Action;
use crate::consts::MAP_HEIGHT;
use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::PlayRestriction;
use crate::entity::get_card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::events::EventKind;
use crate::game::GameState;
use crate::game::Location;
use crate::map::edge_indices;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::active_modifier_kinds;
use crate::modifier::has_modifier;
use crate::modifier::modifier_is_buff;
use crate::modifier::modifier_kind_from_u8;
use crate::modifier::modifier_stacks;
use crate::modifier::stacks_max_for;
use crate::relics::iter_owned_relics;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::MonsterEncounter;
use crate::types::MonsterName;
use crate::types::PotionName;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;
use crate::utils::has_relic;
use crate::utils::scale_attack_damage;
use crate::utils::scale_block_gain;
use crate::utils::vuln_factor;
use crate::utils::weak_factor;

#[pyclass(eq, eq_int, frozen, name = "CardKind", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardKind {
    Attack,
    Skill,
    Power,
    Curse,
    Status,
}

impl From<CardKind> for PyCardKind {
    fn from(kind: CardKind) -> Self {
        match kind {
            CardKind::Attack => Self::Attack,
            CardKind::Skill => Self::Skill,
            CardKind::Power => Self::Power,
            CardKind::Curse => Self::Curse,
            CardKind::Status => Self::Status,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "CardColor", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardColor {
    Green,
    Colorless,
    Curse,
}

impl From<CardColor> for PyCardColor {
    fn from(color: CardColor) -> Self {
        match color {
            CardColor::Green => Self::Green,
            CardColor::Colorless => Self::Colorless,
            CardColor::Curse => Self::Curse,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "CardRarity", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

impl From<CardRarity> for PyCardRarity {
    fn from(rarity: CardRarity) -> Self {
        match rarity {
            CardRarity::Basic => Self::Basic,
            CardRarity::Common => Self::Common,
            CardRarity::Uncommon => Self::Uncommon,
            CardRarity::Rare => Self::Rare,
            CardRarity::Special => Self::Special,
            CardRarity::Curse => Self::Curse,
        }
    }
}

// Complex enums are exposed as one flat pyclass per variant. The Rust enum
// survives for composition; variant_union! gives it IntoPyObject dispatch plus
// a union OUTPUT_TYPE so generated stubs type fields as `VariantA | VariantB | ...`
macro_rules! variant_union {
    ($enum:ident { $($variant:ident => $cls:ident),+ $(,)? }) => {
        impl<'py> IntoPyObject<'py> for $enum {
            type Target = PyAny;
            type Output = Bound<'py, PyAny>;
            type Error = PyErr;
            const OUTPUT_TYPE: PyStaticExpr =
                type_hint_union!($(<$cls as PyTypeInfo>::TYPE_HINT),+);
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                Ok(match self {
                    $( Self::$variant(v) => Bound::new(py, v)?.into_any(), )+
                })
            }
        }
    };
}

#[pyclass(eq, hash, frozen, name = "CardCostKindFixed", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindFixed;

#[pyclass(
    eq,
    hash,
    frozen,
    name = "CardCostKindMinusDiscardsThisTurn",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindMinusDiscardsThisTurn;

#[pyclass(
    eq,
    hash,
    frozen,
    name = "CardCostKindGrowsOnDamageInstanceTaken",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindGrowsOnDamageInstanceTaken;

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "CardCostKindXCost",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCardCostKindXCost {
    pub offset: i8,
}

// NB: variant order and field order must stay byte-identical to the internal
// enum — card_identity_hash feeds this through derived Hash
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyCardCostKind {
    Fixed(PyCardCostKindFixed),
    MinusDiscardsThisTurn(PyCardCostKindMinusDiscardsThisTurn),
    GrowsOnDamageInstanceTaken(PyCardCostKindGrowsOnDamageInstanceTaken),
    XCost(PyCardCostKindXCost),
}

variant_union!(PyCardCostKind {
    Fixed => PyCardCostKindFixed,
    MinusDiscardsThisTurn => PyCardCostKindMinusDiscardsThisTurn,
    GrowsOnDamageInstanceTaken => PyCardCostKindGrowsOnDamageInstanceTaken,
    XCost => PyCardCostKindXCost,
});

impl From<CardCostKind> for PyCardCostKind {
    fn from(kind: CardCostKind) -> Self {
        match kind {
            CardCostKind::Fixed => Self::Fixed(PyCardCostKindFixed),
            CardCostKind::MinusDiscardsThisTurn => {
                Self::MinusDiscardsThisTurn(PyCardCostKindMinusDiscardsThisTurn)
            }
            CardCostKind::GrowsOnDamageInstanceTaken => {
                Self::GrowsOnDamageInstanceTaken(PyCardCostKindGrowsOnDamageInstanceTaken)
            }
            CardCostKind::XCost { offset } => Self::XCost(PyCardCostKindXCost { offset }),
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "PlayRestriction", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyPlayRestriction {
    Always,
    Never,
    DrawPileEmpty,
}

impl From<PlayRestriction> for PyPlayRestriction {
    fn from(restriction: PlayRestriction) -> Self {
        match restriction {
            PlayRestriction::Always => Self::Always,
            PlayRestriction::Never => Self::Never,
            PlayRestriction::DrawPileEmpty => Self::DrawPileEmpty,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "DeltaSign", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyDeltaSign {
    Gain,
    Loss,
}

impl From<DeltaSign> for PyDeltaSign {
    fn from(sign: DeltaSign) -> Self {
        match sign {
            DeltaSign::Gain => Self::Gain,
            DeltaSign::Loss => Self::Loss,
        }
    }
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "AmountAbsolute",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountAbsolute {
    pub amount: u16,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "AmountRelative",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountRelative {
    pub numerator: u8,
    pub denominator: u8,
}

#[pyclass(eq, hash, frozen, get_all, name = "AmountRange", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountRange {
    pub min: u16,
    pub max: u16,
}

#[pyclass(eq, hash, frozen, name = "AmountEventGoldAsk", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountEventGoldAsk;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyAmount {
    Absolute(PyAmountAbsolute),
    Relative(PyAmountRelative),
    Range(PyAmountRange),
    EventGoldAsk(PyAmountEventGoldAsk),
}

variant_union!(PyAmount {
    Absolute => PyAmountAbsolute,
    Relative => PyAmountRelative,
    Range => PyAmountRange,
    EventGoldAsk => PyAmountEventGoldAsk,
});

impl From<Amount> for PyAmount {
    fn from(amount: Amount) -> Self {
        match amount {
            Amount::Absolute(amount) => Self::Absolute(PyAmountAbsolute { amount }),
            // Rounding mode is engine-internal; the view keeps one Relative shape
            Amount::Relative {
                numerator,
                denominator,
            }
            | Amount::RelativeRounded {
                numerator,
                denominator,
            }
            | Amount::RelativeCeil {
                numerator,
                denominator,
            } => Self::Relative(PyAmountRelative {
                numerator,
                denominator,
            }),
            Amount::Range { min, max } => Self::Range(PyAmountRange { min, max }),
            Amount::EventGoldAsk => Self::EventGoldAsk(PyAmountEventGoldAsk),
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "RoomKind", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyRoomKind {
    CombatMonster,
    CombatElite,
    CombatBoss,
    RestSite,
    Treasure,
    EventRoom,
    Shop,
    Unknown,
}

impl From<RoomKind> for PyRoomKind {
    fn from(kind: RoomKind) -> Self {
        match kind {
            RoomKind::CombatMonster => Self::CombatMonster,
            RoomKind::CombatElite => Self::CombatElite,
            RoomKind::CombatBoss => Self::CombatBoss,
            RoomKind::RestSite => Self::RestSite,
            RoomKind::Treasure => Self::Treasure,
            RoomKind::EventRoom => Self::EventRoom,
            RoomKind::Shop => Self::Shop,
            RoomKind::Unknown => Self::Unknown,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "PotionName", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyPotionName {
    EnergyPotion,
    BlockPotion,
    StrengthPotion,
    DexterityPotion,
    FirePotion,
    ExplosivePotion,
    WeakPotion,
    FearPotion,
    PoisonPotion,
    SwiftPotion,
    AttackPotion,
    SkillPotion,
    PowerPotion,
    FruitJuice,
}

impl From<PotionName> for PyPotionName {
    fn from(name: PotionName) -> Self {
        match name {
            PotionName::EnergyPotion => Self::EnergyPotion,
            PotionName::BlockPotion => Self::BlockPotion,
            PotionName::StrengthPotion => Self::StrengthPotion,
            PotionName::DexterityPotion => Self::DexterityPotion,
            PotionName::FirePotion => Self::FirePotion,
            PotionName::ExplosivePotion => Self::ExplosivePotion,
            PotionName::WeakPotion => Self::WeakPotion,
            PotionName::FearPotion => Self::FearPotion,
            PotionName::PoisonPotion => Self::PoisonPotion,
            PotionName::SwiftPotion => Self::SwiftPotion,
            PotionName::AttackPotion => Self::AttackPotion,
            PotionName::SkillPotion => Self::SkillPotion,
            PotionName::PowerPotion => Self::PowerPotion,
            PotionName::FruitJuice => Self::FruitJuice,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "PotionRarity", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyPotionRarity {
    Common,
    Uncommon,
    Rare,
}

impl From<PotionRarity> for PyPotionRarity {
    fn from(rarity: PotionRarity) -> Self {
        match rarity {
            PotionRarity::Common => Self::Common,
            PotionRarity::Uncommon => Self::Uncommon,
            PotionRarity::Rare => Self::Rare,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "RelicName", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyRelicName {
    SnakeRing,
    Akabeko,
    Anchor,
    BagOfMarbles,
    BagOfPreparation,
    BloodVial,
    BronzeScales,
    Kunai,
    NinjaScroll,
    OddlySmoothStone,
    Shuriken,
    ThreadAndNeedle,
    TwistedFunnel,
    Vajra,
    Circlet,
    GoldenIdol,
    Lantern,
    ClockworkSouvenir,
    GremlinVisage,
    RedMask,
    Nunchaku,
    InkBottle,
    LetterOpener,
    OrnamentalFan,
    BirdFacedUrn,
    MummifiedHand,
    OrangePellets,
    StrangeSpoon,
    ChemicalX,
    ArtOfWar,
    Orichalcum,
    Pocketwatch,
    StoneCalendar,
    Abacus,
    Sundial,
    WhiteBeastStatue,
    DollysMirror,
    LeesWaffle,
    HappyFlower,
    IncenseBurner,
    MercuryHourglass,
    HornCleat,
    CaptainsWheel,
    Calipers,
    IceCream,
    SneckoSkull,
    Ginger,
    Turnip,
    Tingsha,
    ToughBandages,
    GremlinHorn,
    TheSpecimen,
    LizardTail,
    Boot,
    Torii,
    TungstenRod,
    HandDrill,
    StrikeDummy,
    PaperKrane,
    CentennialPuzzle,
    MealTicket,
    MawBank,
    JuzuBracelet,
    TinyChest,
    EternalFeather,
    AncientTeaSet,
    RegalPillow,
    MeatOnTheBone,
    Omamori,
    DarkstonePeriapt,
    CeramicFish,
    FrozenEgg,
    MoltenEgg,
    ToxicEgg,
    ToyOrnithopter,
    SmilingMask,
    DeadBranch,
    DuVuDoll,
    Pantograph,
    SlingOfCourage,
    Strawberry,
    Pear,
    Mango,
    OldCoin,
    PotionBelt,
    WarPaint,
    Whetstone,
    EmptyCage,
    PandorasBox,
    PenNib,
    FossilizedHelix,
    PreservedInsect,
    UnceasingTop,
    BlueCandle,
    MedicalKit,
    SpiritPoop,
    WarpedTongs,
    CultistHeadpiece,
    FaceOfCleric,
    NlothsHungryFace,
    SsserpentHead,
    OddMushroom,
}

impl From<RelicName> for PyRelicName {
    fn from(name: RelicName) -> Self {
        match name {
            RelicName::SnakeRing => Self::SnakeRing,
            RelicName::Akabeko => Self::Akabeko,
            RelicName::Anchor => Self::Anchor,
            RelicName::BagOfMarbles => Self::BagOfMarbles,
            RelicName::BagOfPreparation => Self::BagOfPreparation,
            RelicName::BloodVial => Self::BloodVial,
            RelicName::BronzeScales => Self::BronzeScales,
            RelicName::Kunai => Self::Kunai,
            RelicName::NinjaScroll => Self::NinjaScroll,
            RelicName::OddlySmoothStone => Self::OddlySmoothStone,
            RelicName::Shuriken => Self::Shuriken,
            RelicName::ThreadAndNeedle => Self::ThreadAndNeedle,
            RelicName::TwistedFunnel => Self::TwistedFunnel,
            RelicName::Vajra => Self::Vajra,
            RelicName::Circlet => Self::Circlet,
            RelicName::GoldenIdol => Self::GoldenIdol,
            RelicName::Lantern => Self::Lantern,
            RelicName::ClockworkSouvenir => Self::ClockworkSouvenir,
            RelicName::GremlinVisage => Self::GremlinVisage,
            RelicName::RedMask => Self::RedMask,
            RelicName::Nunchaku => Self::Nunchaku,
            RelicName::InkBottle => Self::InkBottle,
            RelicName::LetterOpener => Self::LetterOpener,
            RelicName::OrnamentalFan => Self::OrnamentalFan,
            RelicName::BirdFacedUrn => Self::BirdFacedUrn,
            RelicName::MummifiedHand => Self::MummifiedHand,
            RelicName::OrangePellets => Self::OrangePellets,
            RelicName::StrangeSpoon => Self::StrangeSpoon,
            RelicName::ChemicalX => Self::ChemicalX,
            RelicName::ArtOfWar => Self::ArtOfWar,
            RelicName::Orichalcum => Self::Orichalcum,
            RelicName::Pocketwatch => Self::Pocketwatch,
            RelicName::StoneCalendar => Self::StoneCalendar,
            RelicName::Abacus => Self::Abacus,
            RelicName::Sundial => Self::Sundial,
            RelicName::WhiteBeastStatue => Self::WhiteBeastStatue,
            RelicName::DollysMirror => Self::DollysMirror,
            RelicName::LeesWaffle => Self::LeesWaffle,
            RelicName::HappyFlower => Self::HappyFlower,
            RelicName::IncenseBurner => Self::IncenseBurner,
            RelicName::MercuryHourglass => Self::MercuryHourglass,
            RelicName::HornCleat => Self::HornCleat,
            RelicName::CaptainsWheel => Self::CaptainsWheel,
            RelicName::Calipers => Self::Calipers,
            RelicName::IceCream => Self::IceCream,
            RelicName::SneckoSkull => Self::SneckoSkull,
            RelicName::Ginger => Self::Ginger,
            RelicName::Turnip => Self::Turnip,
            RelicName::Tingsha => Self::Tingsha,
            RelicName::ToughBandages => Self::ToughBandages,
            RelicName::GremlinHorn => Self::GremlinHorn,
            RelicName::TheSpecimen => Self::TheSpecimen,
            RelicName::LizardTail => Self::LizardTail,
            RelicName::Boot => Self::Boot,
            RelicName::Torii => Self::Torii,
            RelicName::TungstenRod => Self::TungstenRod,
            RelicName::HandDrill => Self::HandDrill,
            RelicName::StrikeDummy => Self::StrikeDummy,
            RelicName::PaperKrane => Self::PaperKrane,
            RelicName::CentennialPuzzle => Self::CentennialPuzzle,
            RelicName::MealTicket => Self::MealTicket,
            RelicName::MawBank => Self::MawBank,
            RelicName::JuzuBracelet => Self::JuzuBracelet,
            RelicName::TinyChest => Self::TinyChest,
            RelicName::EternalFeather => Self::EternalFeather,
            RelicName::AncientTeaSet => Self::AncientTeaSet,
            RelicName::RegalPillow => Self::RegalPillow,
            RelicName::MeatOnTheBone => Self::MeatOnTheBone,
            RelicName::Omamori => Self::Omamori,
            RelicName::DarkstonePeriapt => Self::DarkstonePeriapt,
            RelicName::CeramicFish => Self::CeramicFish,
            RelicName::FrozenEgg => Self::FrozenEgg,
            RelicName::MoltenEgg => Self::MoltenEgg,
            RelicName::ToxicEgg => Self::ToxicEgg,
            RelicName::ToyOrnithopter => Self::ToyOrnithopter,
            RelicName::SmilingMask => Self::SmilingMask,
            RelicName::DeadBranch => Self::DeadBranch,
            RelicName::DuVuDoll => Self::DuVuDoll,
            RelicName::Pantograph => Self::Pantograph,
            RelicName::SlingOfCourage => Self::SlingOfCourage,
            RelicName::Strawberry => Self::Strawberry,
            RelicName::Pear => Self::Pear,
            RelicName::Mango => Self::Mango,
            RelicName::OldCoin => Self::OldCoin,
            RelicName::PotionBelt => Self::PotionBelt,
            RelicName::WarPaint => Self::WarPaint,
            RelicName::Whetstone => Self::Whetstone,
            RelicName::EmptyCage => Self::EmptyCage,
            RelicName::PandorasBox => Self::PandorasBox,
            RelicName::PenNib => Self::PenNib,
            RelicName::FossilizedHelix => Self::FossilizedHelix,
            RelicName::PreservedInsect => Self::PreservedInsect,
            RelicName::UnceasingTop => Self::UnceasingTop,
            RelicName::BlueCandle => Self::BlueCandle,
            RelicName::MedicalKit => Self::MedicalKit,
            RelicName::SpiritPoop => Self::SpiritPoop,
            RelicName::WarpedTongs => Self::WarpedTongs,
            RelicName::CultistHeadpiece => Self::CultistHeadpiece,
            RelicName::FaceOfCleric => Self::FaceOfCleric,
            RelicName::NlothsHungryFace => Self::NlothsHungryFace,
            RelicName::SsserpentHead => Self::SsserpentHead,
            RelicName::OddMushroom => Self::OddMushroom,
        }
    }
}

impl From<PyRelicName> for RelicName {
    fn from(name: PyRelicName) -> Self {
        match name {
            PyRelicName::SnakeRing => Self::SnakeRing,
            PyRelicName::Akabeko => Self::Akabeko,
            PyRelicName::Anchor => Self::Anchor,
            PyRelicName::BagOfMarbles => Self::BagOfMarbles,
            PyRelicName::BagOfPreparation => Self::BagOfPreparation,
            PyRelicName::BloodVial => Self::BloodVial,
            PyRelicName::BronzeScales => Self::BronzeScales,
            PyRelicName::Kunai => Self::Kunai,
            PyRelicName::NinjaScroll => Self::NinjaScroll,
            PyRelicName::OddlySmoothStone => Self::OddlySmoothStone,
            PyRelicName::Shuriken => Self::Shuriken,
            PyRelicName::ThreadAndNeedle => Self::ThreadAndNeedle,
            PyRelicName::TwistedFunnel => Self::TwistedFunnel,
            PyRelicName::Vajra => Self::Vajra,
            PyRelicName::Circlet => Self::Circlet,
            PyRelicName::GoldenIdol => Self::GoldenIdol,
            PyRelicName::Lantern => Self::Lantern,
            PyRelicName::ClockworkSouvenir => Self::ClockworkSouvenir,
            PyRelicName::GremlinVisage => Self::GremlinVisage,
            PyRelicName::RedMask => Self::RedMask,
            PyRelicName::Nunchaku => Self::Nunchaku,
            PyRelicName::InkBottle => Self::InkBottle,
            PyRelicName::LetterOpener => Self::LetterOpener,
            PyRelicName::OrnamentalFan => Self::OrnamentalFan,
            PyRelicName::BirdFacedUrn => Self::BirdFacedUrn,
            PyRelicName::MummifiedHand => Self::MummifiedHand,
            PyRelicName::OrangePellets => Self::OrangePellets,
            PyRelicName::StrangeSpoon => Self::StrangeSpoon,
            PyRelicName::ChemicalX => Self::ChemicalX,
            PyRelicName::ArtOfWar => Self::ArtOfWar,
            PyRelicName::Orichalcum => Self::Orichalcum,
            PyRelicName::Pocketwatch => Self::Pocketwatch,
            PyRelicName::StoneCalendar => Self::StoneCalendar,
            PyRelicName::Abacus => Self::Abacus,
            PyRelicName::Sundial => Self::Sundial,
            PyRelicName::WhiteBeastStatue => Self::WhiteBeastStatue,
            PyRelicName::DollysMirror => Self::DollysMirror,
            PyRelicName::LeesWaffle => Self::LeesWaffle,
            PyRelicName::HappyFlower => Self::HappyFlower,
            PyRelicName::IncenseBurner => Self::IncenseBurner,
            PyRelicName::MercuryHourglass => Self::MercuryHourglass,
            PyRelicName::HornCleat => Self::HornCleat,
            PyRelicName::CaptainsWheel => Self::CaptainsWheel,
            PyRelicName::Calipers => Self::Calipers,
            PyRelicName::IceCream => Self::IceCream,
            PyRelicName::SneckoSkull => Self::SneckoSkull,
            PyRelicName::Ginger => Self::Ginger,
            PyRelicName::Turnip => Self::Turnip,
            PyRelicName::Tingsha => Self::Tingsha,
            PyRelicName::ToughBandages => Self::ToughBandages,
            PyRelicName::GremlinHorn => Self::GremlinHorn,
            PyRelicName::TheSpecimen => Self::TheSpecimen,
            PyRelicName::LizardTail => Self::LizardTail,
            PyRelicName::Boot => Self::Boot,
            PyRelicName::Torii => Self::Torii,
            PyRelicName::TungstenRod => Self::TungstenRod,
            PyRelicName::HandDrill => Self::HandDrill,
            PyRelicName::StrikeDummy => Self::StrikeDummy,
            PyRelicName::PaperKrane => Self::PaperKrane,
            PyRelicName::CentennialPuzzle => Self::CentennialPuzzle,
            PyRelicName::MealTicket => Self::MealTicket,
            PyRelicName::MawBank => Self::MawBank,
            PyRelicName::JuzuBracelet => Self::JuzuBracelet,
            PyRelicName::TinyChest => Self::TinyChest,
            PyRelicName::EternalFeather => Self::EternalFeather,
            PyRelicName::AncientTeaSet => Self::AncientTeaSet,
            PyRelicName::RegalPillow => Self::RegalPillow,
            PyRelicName::MeatOnTheBone => Self::MeatOnTheBone,
            PyRelicName::Omamori => Self::Omamori,
            PyRelicName::DarkstonePeriapt => Self::DarkstonePeriapt,
            PyRelicName::CeramicFish => Self::CeramicFish,
            PyRelicName::FrozenEgg => Self::FrozenEgg,
            PyRelicName::MoltenEgg => Self::MoltenEgg,
            PyRelicName::ToxicEgg => Self::ToxicEgg,
            PyRelicName::ToyOrnithopter => Self::ToyOrnithopter,
            PyRelicName::SmilingMask => Self::SmilingMask,
            PyRelicName::DeadBranch => Self::DeadBranch,
            PyRelicName::DuVuDoll => Self::DuVuDoll,
            PyRelicName::Pantograph => Self::Pantograph,
            PyRelicName::SlingOfCourage => Self::SlingOfCourage,
            PyRelicName::Strawberry => Self::Strawberry,
            PyRelicName::Pear => Self::Pear,
            PyRelicName::Mango => Self::Mango,
            PyRelicName::OldCoin => Self::OldCoin,
            PyRelicName::PotionBelt => Self::PotionBelt,
            PyRelicName::WarPaint => Self::WarPaint,
            PyRelicName::Whetstone => Self::Whetstone,
            PyRelicName::EmptyCage => Self::EmptyCage,
            PyRelicName::PandorasBox => Self::PandorasBox,
            PyRelicName::PenNib => Self::PenNib,
            PyRelicName::FossilizedHelix => Self::FossilizedHelix,
            PyRelicName::PreservedInsect => Self::PreservedInsect,
            PyRelicName::UnceasingTop => Self::UnceasingTop,
            PyRelicName::BlueCandle => Self::BlueCandle,
            PyRelicName::MedicalKit => Self::MedicalKit,
            PyRelicName::SpiritPoop => Self::SpiritPoop,
            PyRelicName::WarpedTongs => Self::WarpedTongs,
            PyRelicName::CultistHeadpiece => Self::CultistHeadpiece,
            PyRelicName::FaceOfCleric => Self::FaceOfCleric,
            PyRelicName::NlothsHungryFace => Self::NlothsHungryFace,
            PyRelicName::SsserpentHead => Self::SsserpentHead,
            PyRelicName::OddMushroom => Self::OddMushroom,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "CardName", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCardName {
    AThousandCuts,
    Accuracy,
    Acrobatics,
    Adrenaline,
    AfterImage,
    Alchemize,
    AllOutAttack,
    Backflip,
    Backstab,
    BandageUp,
    Bane,
    BladeDance,
    Blind,
    Blur,
    BouncingFlask,
    BulletTime,
    Burn,
    Burst,
    CalculatedGamble,
    Caltrops,
    Catalyst,
    Choke,
    CloakAndDagger,
    Concentrate,
    CorpseExplosion,
    CripplingPoison,
    DaggerSpray,
    DaggerThrow,
    Dash,
    Dazed,
    DeadlyPoison,
    DeepBreath,
    Defend,
    Deflect,
    DieDieDie,
    Distraction,
    DodgeAndRoll,
    Doppelganger,
    EndlessAgony,
    Envenom,
    EscapePlan,
    Eviscerate,
    Expertise,
    Finesse,
    Finisher,
    FlashOfSteel,
    Flechettes,
    FlyingKnee,
    Footwork,
    GlassKnife,
    GoodInstincts,
    GrandFinale,
    HeelHook,
    InfiniteBlades,
    LegSweep,
    Malaise,
    MasterOfStrategy,
    MasterfulStab,
    MindBlast,
    Neutralize,
    Nightmare,
    NoxiousFumes,
    Outmaneuver,
    PhantasmalKiller,
    PiercingWail,
    PoisonedStab,
    Predator,
    Prepared,
    QuickSlash,
    Reflex,
    RiddleWithHoles,
    Setup,
    Shiv,
    Skewer,
    Slice,
    Slimed,
    SneakyStrike,
    StormOfSteel,
    Strike,
    SuckerPunch,
    Survivor,
    SwiftStrike,
    Tactician,
    Terror,
    ToolsOfTheTrade,
    Unload,
    WellLaidPlans,
    WraithForm,
    AscendersBane,
    Regret,
    Pain,
    Doubt,
    Decay,
    Injury,
    Shame,
    Writhe,
    Parasite,
    Normality,
}

impl From<CardName> for PyCardName {
    // 1:1 by name; explicit match (not transmute) catches drift if either enum changes
    fn from(name: CardName) -> Self {
        match name {
            CardName::AThousandCuts => Self::AThousandCuts,
            CardName::Accuracy => Self::Accuracy,
            CardName::Acrobatics => Self::Acrobatics,
            CardName::Adrenaline => Self::Adrenaline,
            CardName::AfterImage => Self::AfterImage,
            CardName::Alchemize => Self::Alchemize,
            CardName::AllOutAttack => Self::AllOutAttack,
            CardName::Backflip => Self::Backflip,
            CardName::Backstab => Self::Backstab,
            CardName::BandageUp => Self::BandageUp,
            CardName::Bane => Self::Bane,
            CardName::BladeDance => Self::BladeDance,
            CardName::Blind => Self::Blind,
            CardName::Blur => Self::Blur,
            CardName::BouncingFlask => Self::BouncingFlask,
            CardName::BulletTime => Self::BulletTime,
            CardName::Burn => Self::Burn,
            CardName::Burst => Self::Burst,
            CardName::CalculatedGamble => Self::CalculatedGamble,
            CardName::Caltrops => Self::Caltrops,
            CardName::Catalyst => Self::Catalyst,
            CardName::Choke => Self::Choke,
            CardName::CloakAndDagger => Self::CloakAndDagger,
            CardName::Concentrate => Self::Concentrate,
            CardName::CorpseExplosion => Self::CorpseExplosion,
            CardName::CripplingPoison => Self::CripplingPoison,
            CardName::DaggerSpray => Self::DaggerSpray,
            CardName::DaggerThrow => Self::DaggerThrow,
            CardName::Dash => Self::Dash,
            CardName::Dazed => Self::Dazed,
            CardName::DeadlyPoison => Self::DeadlyPoison,
            CardName::DeepBreath => Self::DeepBreath,
            CardName::Defend => Self::Defend,
            CardName::Deflect => Self::Deflect,
            CardName::DieDieDie => Self::DieDieDie,
            CardName::Distraction => Self::Distraction,
            CardName::DodgeAndRoll => Self::DodgeAndRoll,
            CardName::Doppelganger => Self::Doppelganger,
            CardName::EndlessAgony => Self::EndlessAgony,
            CardName::Envenom => Self::Envenom,
            CardName::EscapePlan => Self::EscapePlan,
            CardName::Eviscerate => Self::Eviscerate,
            CardName::Expertise => Self::Expertise,
            CardName::Finesse => Self::Finesse,
            CardName::Finisher => Self::Finisher,
            CardName::FlashOfSteel => Self::FlashOfSteel,
            CardName::Flechettes => Self::Flechettes,
            CardName::FlyingKnee => Self::FlyingKnee,
            CardName::Footwork => Self::Footwork,
            CardName::GlassKnife => Self::GlassKnife,
            CardName::GoodInstincts => Self::GoodInstincts,
            CardName::GrandFinale => Self::GrandFinale,
            CardName::HeelHook => Self::HeelHook,
            CardName::InfiniteBlades => Self::InfiniteBlades,
            CardName::LegSweep => Self::LegSweep,
            CardName::Malaise => Self::Malaise,
            CardName::MasterOfStrategy => Self::MasterOfStrategy,
            CardName::MasterfulStab => Self::MasterfulStab,
            CardName::MindBlast => Self::MindBlast,
            CardName::Neutralize => Self::Neutralize,
            CardName::Nightmare => Self::Nightmare,
            CardName::NoxiousFumes => Self::NoxiousFumes,
            CardName::Outmaneuver => Self::Outmaneuver,
            CardName::PhantasmalKiller => Self::PhantasmalKiller,
            CardName::PiercingWail => Self::PiercingWail,
            CardName::PoisonedStab => Self::PoisonedStab,
            CardName::Predator => Self::Predator,
            CardName::Prepared => Self::Prepared,
            CardName::QuickSlash => Self::QuickSlash,
            CardName::Reflex => Self::Reflex,
            CardName::RiddleWithHoles => Self::RiddleWithHoles,
            CardName::Setup => Self::Setup,
            CardName::Shiv => Self::Shiv,
            CardName::Skewer => Self::Skewer,
            CardName::Slice => Self::Slice,
            CardName::Slimed => Self::Slimed,
            CardName::SneakyStrike => Self::SneakyStrike,
            CardName::StormOfSteel => Self::StormOfSteel,
            CardName::Strike => Self::Strike,
            CardName::SuckerPunch => Self::SuckerPunch,
            CardName::Survivor => Self::Survivor,
            CardName::SwiftStrike => Self::SwiftStrike,
            CardName::Tactician => Self::Tactician,
            CardName::Terror => Self::Terror,
            CardName::ToolsOfTheTrade => Self::ToolsOfTheTrade,
            CardName::Unload => Self::Unload,
            CardName::WellLaidPlans => Self::WellLaidPlans,
            CardName::WraithForm => Self::WraithForm,
            CardName::AscendersBane => Self::AscendersBane,
            CardName::Regret => Self::Regret,
            CardName::Pain => Self::Pain,
            CardName::Doubt => Self::Doubt,
            CardName::Decay => Self::Decay,
            CardName::Injury => Self::Injury,
            CardName::Shame => Self::Shame,
            CardName::Writhe => Self::Writhe,
            CardName::Parasite => Self::Parasite,
            CardName::Normality => Self::Normality,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "MonsterName", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyMonsterName {
    Cultist,
    FungiBeast,
    GremlinFat,
    GremlinNob,
    GremlinThief,
    GremlinTsundere,
    GremlinWarrior,
    GremlinWizard,
    Hexaghost,
    JawWorm,
    Lagavulin,
    Looter,
    LouseDefensive,
    LouseNormal,
    Sentry,
    SlaverBlue,
    SlaverRed,
    SlimeAcidLarge,
    SlimeAcidMedium,
    SlimeAcidSmall,
    SlimeBoss,
    SlimeSpikeLarge,
    SlimeSpikeMedium,
    SlimeSpikeSmall,
    TheGuardian,
}

impl From<MonsterName> for PyMonsterName {
    fn from(name: MonsterName) -> Self {
        match name {
            MonsterName::Cultist => Self::Cultist,
            MonsterName::FungiBeast => Self::FungiBeast,
            MonsterName::GremlinFat => Self::GremlinFat,
            MonsterName::GremlinNob => Self::GremlinNob,
            MonsterName::GremlinThief => Self::GremlinThief,
            MonsterName::GremlinTsundere => Self::GremlinTsundere,
            MonsterName::GremlinWarrior => Self::GremlinWarrior,
            MonsterName::GremlinWizard => Self::GremlinWizard,
            MonsterName::Hexaghost => Self::Hexaghost,
            MonsterName::JawWorm => Self::JawWorm,
            MonsterName::Lagavulin => Self::Lagavulin,
            MonsterName::Looter => Self::Looter,
            MonsterName::LouseDefensive => Self::LouseDefensive,
            MonsterName::LouseNormal => Self::LouseNormal,
            MonsterName::Sentry => Self::Sentry,
            MonsterName::SlaverBlue => Self::SlaverBlue,
            MonsterName::SlaverRed => Self::SlaverRed,
            MonsterName::SlimeAcidLarge => Self::SlimeAcidLarge,
            MonsterName::SlimeAcidMedium => Self::SlimeAcidMedium,
            MonsterName::SlimeAcidSmall => Self::SlimeAcidSmall,
            MonsterName::SlimeBoss => Self::SlimeBoss,
            MonsterName::SlimeSpikeLarge => Self::SlimeSpikeLarge,
            MonsterName::SlimeSpikeMedium => Self::SlimeSpikeMedium,
            MonsterName::SlimeSpikeSmall => Self::SlimeSpikeSmall,
            MonsterName::TheGuardian => Self::TheGuardian,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "MonsterEncounter", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyMonsterEncounter {
    Cultist,
    JawWorm,
    TwoLouse,
    SmallSlimes,
    BlueSlaver,
    RedSlaver,
    Looter,
    TwoFungiBeasts,
    ThreeLouse,
    LargeSlime,
    LotsOfSlimes,
    GremlinGang,
    ExordiumThugs,
    ExordiumWildlife,
    GremlinNob,
    Lagavulin,
    ThreeSentries,
    TheGuardian,
    Hexaghost,
    SlimeBoss,
    ThreeFungiBeasts,
}

impl From<MonsterEncounter> for PyMonsterEncounter {
    fn from(e: MonsterEncounter) -> Self {
        match e {
            MonsterEncounter::Cultist => Self::Cultist,
            MonsterEncounter::JawWorm => Self::JawWorm,
            MonsterEncounter::TwoLouse => Self::TwoLouse,
            MonsterEncounter::SmallSlimes => Self::SmallSlimes,
            MonsterEncounter::BlueSlaver => Self::BlueSlaver,
            MonsterEncounter::RedSlaver => Self::RedSlaver,
            MonsterEncounter::Looter => Self::Looter,
            MonsterEncounter::TwoFungiBeasts => Self::TwoFungiBeasts,
            MonsterEncounter::ThreeLouse => Self::ThreeLouse,
            MonsterEncounter::LargeSlime => Self::LargeSlime,
            MonsterEncounter::LotsOfSlimes => Self::LotsOfSlimes,
            MonsterEncounter::GremlinGang => Self::GremlinGang,
            MonsterEncounter::ExordiumThugs => Self::ExordiumThugs,
            MonsterEncounter::ExordiumWildlife => Self::ExordiumWildlife,
            MonsterEncounter::GremlinNob => Self::GremlinNob,
            MonsterEncounter::Lagavulin => Self::Lagavulin,
            MonsterEncounter::ThreeSentries => Self::ThreeSentries,
            MonsterEncounter::TheGuardian => Self::TheGuardian,
            MonsterEncounter::Hexaghost => Self::Hexaghost,
            MonsterEncounter::SlimeBoss => Self::SlimeBoss,
            MonsterEncounter::ThreeFungiBeasts => Self::ThreeFungiBeasts,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "RelicTier", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyRelicTier {
    Starter,
    Common,
    Uncommon,
    Rare,
    Boss,
    Shop,
    Special,
}

impl From<RelicTier> for PyRelicTier {
    fn from(tier: RelicTier) -> Self {
        match tier {
            RelicTier::Starter => Self::Starter,
            RelicTier::Common => Self::Common,
            RelicTier::Uncommon => Self::Uncommon,
            RelicTier::Rare => Self::Rare,
            RelicTier::Boss => Self::Boss,
            RelicTier::Shop => Self::Shop,
            RelicTier::Special => Self::Special,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "ModifierKind", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyModifierKind {
    Accuracy,
    AfterImage,
    Angry,
    Artifact,
    Asleep,
    Blur,
    Burst,
    Choke,
    CorpseExplosion,
    CurlUp,
    Dexterity,
    DoubleDamage,
    DrawCardNextTurn,
    Enrage,
    Entangled,
    Envenom,
    Frail,
    InfiniteBlades,
    Intangible,
    Metallicize,
    ModeShift,
    NextTurnBlock,
    NextTurnEnergy,
    NoDraw,
    NoxiousFumes,
    Phantasmal,
    PlatedArmor,
    Poison,
    Retain,
    Ritual,
    Shackled,
    SharpHide,
    Splittable,
    SporeCloud,
    Strength,
    Thievery,
    Thorns,
    ThousandCuts,
    ToolsOfTheTrade,
    Vigor,
    Vulnerable,
    Weak,
    WraithForm,
    Buffer,
    PenNib,
}

#[pymethods]
impl PyModifierKind {
    #[getter]
    fn is_buff(&self) -> bool {
        modifier_is_buff(modifier_kind_from_u8(*self as u8))
    }

    // hash by discriminant (other unit enums get this via impl_discriminant_hash)
    fn __hash__(&self) -> isize {
        *self as isize
    }
}

impl From<ModifierKind> for PyModifierKind {
    fn from(kind: ModifierKind) -> Self {
        match kind {
            ModifierKind::Accuracy => Self::Accuracy,
            ModifierKind::AfterImage => Self::AfterImage,
            ModifierKind::Angry => Self::Angry,
            ModifierKind::Artifact => Self::Artifact,
            ModifierKind::Asleep => Self::Asleep,
            ModifierKind::Blur => Self::Blur,
            ModifierKind::Burst => Self::Burst,
            ModifierKind::Choke => Self::Choke,
            ModifierKind::CorpseExplosion => Self::CorpseExplosion,
            ModifierKind::CurlUp => Self::CurlUp,
            ModifierKind::Dexterity => Self::Dexterity,
            ModifierKind::DoubleDamage => Self::DoubleDamage,
            ModifierKind::DrawCardNextTurn => Self::DrawCardNextTurn,
            ModifierKind::Enrage => Self::Enrage,
            ModifierKind::Entangled => Self::Entangled,
            ModifierKind::Envenom => Self::Envenom,
            ModifierKind::Frail => Self::Frail,
            ModifierKind::InfiniteBlades => Self::InfiniteBlades,
            ModifierKind::Intangible => Self::Intangible,
            ModifierKind::Metallicize => Self::Metallicize,
            ModifierKind::ModeShift => Self::ModeShift,
            ModifierKind::NextTurnBlock => Self::NextTurnBlock,
            ModifierKind::NextTurnEnergy => Self::NextTurnEnergy,
            ModifierKind::NoDraw => Self::NoDraw,
            ModifierKind::NoxiousFumes => Self::NoxiousFumes,
            ModifierKind::Phantasmal => Self::Phantasmal,
            ModifierKind::PlatedArmor => Self::PlatedArmor,
            ModifierKind::Poison => Self::Poison,
            ModifierKind::Retain => Self::Retain,
            ModifierKind::Ritual => Self::Ritual,
            ModifierKind::Shackled => Self::Shackled,
            ModifierKind::SharpHide => Self::SharpHide,
            ModifierKind::Splittable => Self::Splittable,
            ModifierKind::SporeCloud => Self::SporeCloud,
            ModifierKind::Strength => Self::Strength,
            ModifierKind::Thievery => Self::Thievery,
            ModifierKind::Thorns => Self::Thorns,
            ModifierKind::ThousandCuts => Self::ThousandCuts,
            ModifierKind::ToolsOfTheTrade => Self::ToolsOfTheTrade,
            ModifierKind::Vigor => Self::Vigor,
            ModifierKind::Vulnerable => Self::Vulnerable,
            ModifierKind::Weak => Self::Weak,
            ModifierKind::WraithForm => Self::WraithForm,
            ModifierKind::Buffer => Self::Buffer,
            ModifierKind::PenNib => Self::PenNib,
        }
    }
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "CandidatePoolHand",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolHand {
    pub filter: PyCandidatePoolCardFilter,
}

#[pyclass(
    eq,
    hash,
    frozen,
    name = "CandidatePoolCharacter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolCharacter;

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "CandidatePoolMonsters",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolMonsters {
    pub filter: PyCandidatePoolMonstersFilter,
}

#[pyclass(eq, hash, frozen, name = "CandidatePoolSource", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolSource;

#[pyclass(eq, hash, frozen, name = "CandidatePoolDiscover", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolDiscover;

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "CandidatePoolDeck",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolDeck {
    pub filter: PyCandidatePoolCardFilter,
}

#[pyclass(
    eq,
    hash,
    frozen,
    name = "CandidatePoolEventPickCard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolEventPickCard;

#[pyclass(
    eq,
    hash,
    frozen,
    name = "CandidatePoolEventPickPotion",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolEventPickPotion;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyCandidatePool {
    Hand(PyCandidatePoolHand),
    Character(PyCandidatePoolCharacter),
    Monsters(PyCandidatePoolMonsters),
    Source(PyCandidatePoolSource),
    Discover(PyCandidatePoolDiscover),
    Deck(PyCandidatePoolDeck),
    EventPickCard(PyCandidatePoolEventPickCard),
    EventPickPotion(PyCandidatePoolEventPickPotion),
}

variant_union!(PyCandidatePool {
    Hand => PyCandidatePoolHand,
    Character => PyCandidatePoolCharacter,
    Monsters => PyCandidatePoolMonsters,
    Source => PyCandidatePoolSource,
    Discover => PyCandidatePoolDiscover,
    Deck => PyCandidatePoolDeck,
    EventPickCard => PyCandidatePoolEventPickCard,
    EventPickPotion => PyCandidatePoolEventPickPotion,
});

impl From<CandidatePool> for PyCandidatePool {
    fn from(pool: CandidatePool) -> Self {
        match pool {
            CandidatePool::Hand { filter } => Self::Hand(PyCandidatePoolHand {
                filter: filter.into(),
            }),
            CandidatePool::Character => Self::Character(PyCandidatePoolCharacter),
            CandidatePool::Monsters { filter } => Self::Monsters(PyCandidatePoolMonsters {
                filter: filter.into(),
            }),
            CandidatePool::Source => Self::Source(PyCandidatePoolSource),
            CandidatePool::Discover => Self::Discover(PyCandidatePoolDiscover),
            CandidatePool::Deck { filter } => Self::Deck(PyCandidatePoolDeck {
                filter: filter.into(),
            }),
            CandidatePool::EventPickCard => Self::EventPickCard(PyCandidatePoolEventPickCard),
            CandidatePool::EventPickPotion => Self::EventPickPotion(PyCandidatePoolEventPickPotion),
        }
    }
}

#[pyclass(
    eq,
    eq_int,
    frozen,
    name = "CandidatePoolMonstersFilter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCandidatePoolMonstersFilter {
    All,
    Other,
    Picked,
}

impl From<CandidatePoolMonstersFilter> for PyCandidatePoolMonstersFilter {
    fn from(f: CandidatePoolMonstersFilter) -> Self {
        match f {
            CandidatePoolMonstersFilter::All => Self::All,
            CandidatePoolMonstersFilter::Other => Self::Other,
            CandidatePoolMonstersFilter::Picked => Self::Picked,
        }
    }
}

#[pyclass(
    eq,
    eq_int,
    frozen,
    name = "CandidatePoolCardFilter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCandidatePoolCardFilter {
    Purgeable,
    Upgradeable,
    Any,
    Transformable,
    PurgeableCurse,
}

impl From<CandidatePoolCardFilter> for PyCandidatePoolCardFilter {
    fn from(f: CandidatePoolCardFilter) -> Self {
        match f {
            CandidatePoolCardFilter::Purgeable => Self::Purgeable,
            CandidatePoolCardFilter::Upgradeable => Self::Upgradeable,
            CandidatePoolCardFilter::Any => Self::Any,
            CandidatePoolCardFilter::Transformable => Self::Transformable,
            CandidatePoolCardFilter::PurgeableCurse => Self::PurgeableCurse,
        }
    }
}

#[pyclass(eq, hash, frozen, name = "SelectionKindAll", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindAll;

#[pyclass(eq, hash, frozen, name = "SelectionKindSingle", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindSingle;

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "SelectionKindRandom",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindRandom {
    pub count: u8,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "SelectionKindInput",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindInput {
    pub count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PySelectionKind {
    All(PySelectionKindAll),
    Single(PySelectionKindSingle),
    Random(PySelectionKindRandom),
    Input(PySelectionKindInput),
}

variant_union!(PySelectionKind {
    All => PySelectionKindAll,
    Single => PySelectionKindSingle,
    Random => PySelectionKindRandom,
    Input => PySelectionKindInput,
});

impl From<SelectionKind> for PySelectionKind {
    fn from(selection_kind: SelectionKind) -> Self {
        match selection_kind {
            SelectionKind::All => Self::All(PySelectionKindAll),
            SelectionKind::Single => Self::Single(PySelectionKindSingle),
            SelectionKind::Random { count } => Self::Random(PySelectionKindRandom { count }),
            SelectionKind::Input { count } => Self::Input(PySelectionKindInput { count }),
        }
    }
}

#[pyclass(eq, hash, frozen, get_all, name = "Target", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyTarget {
    pub candidate_pool: PyCandidatePool,
    pub selection_kind: PySelectionKind,
}

// `PyActionType` is the discriminant for the flat `PyAction` struct below
#[pyclass(eq, eq_int, frozen, name = "ActionType", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::IntoStaticStr)]
pub enum PyActionType {
    CardDiscard,
    CardDiscover,
    CardDuplicate,
    CardNightmare,
    CardPlay,
    CardPurge,
    CardRetain,
    CardSetup,
    CardTransform,
    CardUpgrade,
    ChestOpen,
    EventOptionSelect,
    PotionDiscard,
    PotionUse,
    Rest,
    RewardTakeCard,
    RewardTakeGold,
    RewardTakePotion,
    RewardTakeRelic,
    RoomExit,
    RoomSelect,
    ShopBuyCard,
    ShopBuyPotion,
    ShopBuyRelic,
    ShopPurge,
    TurnEnd,
}

#[pymethods]
impl PyActionType {
    // variant name for the action-spec registry (raw pyo3 enums have no .name)
    #[getter]
    fn name(&self) -> &'static str {
        self.into()
    }

    // hash by discriminant so eq and hash agree (see impl_discriminant_hash below)
    fn __hash__(&self) -> isize {
        *self as isize
    }
}

impl PyActionType {
    fn from_discriminant(discriminant: u8) -> Result<Self, String> {
        match discriminant {
            0 => Ok(Self::CardDiscard),
            1 => Ok(Self::CardDiscover),
            2 => Ok(Self::CardDuplicate),
            3 => Ok(Self::CardNightmare),
            4 => Ok(Self::CardPlay),
            5 => Ok(Self::CardPurge),
            6 => Ok(Self::CardRetain),
            7 => Ok(Self::CardSetup),
            8 => Ok(Self::CardTransform),
            9 => Ok(Self::CardUpgrade),
            10 => Ok(Self::ChestOpen),
            11 => Ok(Self::EventOptionSelect),
            12 => Ok(Self::PotionDiscard),
            13 => Ok(Self::PotionUse),
            14 => Ok(Self::Rest),
            15 => Ok(Self::RewardTakeCard),
            16 => Ok(Self::RewardTakeGold),
            17 => Ok(Self::RewardTakePotion),
            18 => Ok(Self::RewardTakeRelic),
            19 => Ok(Self::RoomExit),
            20 => Ok(Self::RoomSelect),
            21 => Ok(Self::ShopBuyCard),
            22 => Ok(Self::ShopBuyPotion),
            23 => Ok(Self::ShopBuyRelic),
            24 => Ok(Self::ShopPurge),
            25 => Ok(Self::TurnEnd),
            _ => Err(format!("PyActionType: invalid discriminant {discriminant}")),
        }
    }
}

#[pyclass(eq, hash, frozen, name = "Action", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAction {
    #[pyo3(get)]
    pub action_type: PyActionType,
    #[pyo3(get)]
    pub idxs: Vec<usize>,
    #[pyo3(get)]
    pub kind: Option<u8>,
}

#[pymethods]
impl PyAction {
    #[new]
    #[pyo3(signature = (action_type, idxs, kind=None))]
    fn new(action_type: PyActionType, idxs: Vec<usize>, kind: Option<u8>) -> Self {
        Self {
            action_type,
            idxs,
            kind,
        }
    }

    fn __repr__(&self) -> String {
        match self.kind {
            Some(k) => format!(
                "PyAction({:?}, {:?}, kind={})",
                self.action_type, self.idxs, k
            ),
            None => format!("PyAction({:?}, {:?})", self.action_type, self.idxs),
        }
    }
}

pub fn to_internal_action(action: PyAction) -> Result<Action, String> {
    let idxs = &action.idxs;
    match action.action_type {
        PyActionType::CardPlay => match idxs.len() {
            1 => Ok(Action::CardPlay {
                idx_card: idxs[0],
                idx_monster: None,
            }),
            2 => Ok(Action::CardPlay {
                idx_card: idxs[0],
                idx_monster: Some(idxs[1]),
            }),
            n => Err(format!(
                "CardPlay expects [idx_card] or [idx_card, idx_monster], got {n} idxs"
            )),
        },
        PyActionType::TurnEnd => match idxs.len() {
            0 => Ok(Action::TurnEnd),
            n => Err(format!("TurnEnd expects [], got {n} idxs")),
        },
        PyActionType::CardDiscard => match idxs.len() {
            1 => Ok(Action::CardDiscard { idx: idxs[0] }),
            n => Err(format!("CardDiscard expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardRetain => match idxs.len() {
            1 => Ok(Action::CardRetain { idx: idxs[0] }),
            n => Err(format!("CardRetain expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardSetup => match idxs.len() {
            1 => Ok(Action::CardSetup { idx: idxs[0] }),
            n => Err(format!("CardSetup expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardNightmare => match idxs.len() {
            1 => Ok(Action::CardNightmare { idx: idxs[0] }),
            n => Err(format!("CardNightmare expects [idx_hand], got {n} idxs")),
        },
        PyActionType::RoomSelect => match idxs.len() {
            1 => Ok(Action::RoomSelect { idx: idxs[0] }),
            n => Err(format!("RoomSelect expects [idx], got {n} idxs")),
        },
        PyActionType::ShopBuyCard => match idxs.len() {
            1 => Ok(Action::ShopBuyCard { idx: idxs[0] }),
            n => Err(format!("ShopBuyCard expects [idx], got {n} idxs")),
        },
        PyActionType::ShopBuyPotion => match idxs.len() {
            1 => Ok(Action::ShopBuyPotion { idx: idxs[0] }),
            n => Err(format!("ShopBuyPotion expects [idx], got {n} idxs")),
        },
        PyActionType::ShopBuyRelic => match idxs.len() {
            1 => Ok(Action::ShopBuyRelic { idx: idxs[0] }),
            n => Err(format!("ShopBuyRelic expects [idx], got {n} idxs")),
        },
        PyActionType::ShopPurge => match idxs.len() {
            1 => Ok(Action::ShopPurge { idx: idxs[0] }),
            n => Err(format!("ShopPurge expects [idx], got {n} idxs")),
        },
        PyActionType::Rest => match idxs.len() {
            0 => Ok(Action::Rest),
            n => Err(format!("Rest expects [], got {n} idxs")),
        },
        PyActionType::RoomExit => match idxs.len() {
            0 => Ok(Action::RoomExit),
            n => Err(format!("RoomExit expects [], got {n} idxs")),
        },
        PyActionType::ChestOpen => match idxs.len() {
            0 => Ok(Action::ChestOpen),
            n => Err(format!("ChestOpen expects [], got {n} idxs")),
        },
        PyActionType::PotionUse => match idxs.len() {
            1 => Ok(Action::PotionUse {
                idx_potion: idxs[0],
                idx_monster: None,
            }),
            2 => Ok(Action::PotionUse {
                idx_potion: idxs[0],
                idx_monster: Some(idxs[1]),
            }),
            n => Err(format!(
                "PotionUse expects [idx_potion] or [idx_potion, idx_monster], got {n} idxs"
            )),
        },
        PyActionType::PotionDiscard => match idxs.len() {
            1 => Ok(Action::PotionDiscard { idx: idxs[0] }),
            n => Err(format!("PotionDiscard expects [idx_slot], got {n} idxs")),
        },
        PyActionType::CardDiscover => match idxs.len() {
            1 => Ok(Action::CardDiscover { idx: idxs[0] }),
            n => Err(format!("CardDiscover expects [idx], got {n} idxs")),
        },
        PyActionType::RewardTakeCard => match idxs.len() {
            1 => Ok(Action::RewardTakeCard { idx: idxs[0] }),
            n => Err(format!("RewardTakeCard expects [idx], got {n} idxs")),
        },
        PyActionType::RewardTakeRelic => match idxs.len() {
            0 => Ok(Action::RewardTakeRelic),
            n => Err(format!("RewardTakeRelic expects [], got {n} idxs")),
        },
        PyActionType::RewardTakePotion => match idxs.len() {
            1 => Ok(Action::RewardTakePotion { idx: idxs[0] }),
            n => Err(format!("RewardTakePotion expects [idx], got {n} idxs")),
        },
        PyActionType::RewardTakeGold => match idxs.len() {
            0 => Ok(Action::RewardTakeGold),
            n => Err(format!("RewardTakeGold expects [], got {n} idxs")),
        },
        PyActionType::EventOptionSelect => match idxs.len() {
            1 => Ok(Action::EventOptionSelect { idx: idxs[0] }),
            n => Err(format!("EventOptionSelect expects [idx], got {n} idxs")),
        },
        PyActionType::CardPurge => match idxs.len() {
            1 => Ok(Action::CardPurge { idx: idxs[0] }),
            n => Err(format!("CardPurge expects [idx], got {n} idxs")),
        },
        PyActionType::CardUpgrade => match idxs.len() {
            1 => Ok(Action::CardUpgrade { idx: idxs[0] }),
            n => Err(format!("CardUpgrade expects [idx], got {n} idxs")),
        },
        PyActionType::CardDuplicate => match idxs.len() {
            1 => Ok(Action::CardDuplicate { idx: idxs[0] }),
            n => Err(format!("CardDuplicate expects [idx], got {n} idxs")),
        },
        PyActionType::CardTransform => match idxs.len() {
            1 => Ok(Action::CardTransform { idx: idxs[0] }),
            n => Err(format!("CardTransform expects [idx], got {n} idxs")),
        },
    }
}

pub fn from_internal_action(action: Action) -> PyAction {
    let (action_type, idxs) = match action {
        Action::CardPlay {
            idx_card,
            idx_monster: None,
        } => (PyActionType::CardPlay, vec![idx_card]),
        Action::CardPlay {
            idx_card,
            idx_monster: Some(m),
        } => (PyActionType::CardPlay, vec![idx_card, m]),
        Action::TurnEnd => (PyActionType::TurnEnd, vec![]),
        Action::CardDiscard { idx } => (PyActionType::CardDiscard, vec![idx]),
        Action::CardRetain { idx } => (PyActionType::CardRetain, vec![idx]),
        Action::CardSetup { idx } => (PyActionType::CardSetup, vec![idx]),
        Action::CardNightmare { idx } => (PyActionType::CardNightmare, vec![idx]),
        Action::RoomSelect { idx } => (PyActionType::RoomSelect, vec![idx]),
        Action::Rest => (PyActionType::Rest, vec![]),
        Action::RoomExit => (PyActionType::RoomExit, vec![]),
        Action::ShopBuyCard { idx } => (PyActionType::ShopBuyCard, vec![idx]),
        Action::ShopBuyPotion { idx } => (PyActionType::ShopBuyPotion, vec![idx]),
        Action::ShopBuyRelic { idx } => (PyActionType::ShopBuyRelic, vec![idx]),
        Action::ShopPurge { idx } => (PyActionType::ShopPurge, vec![idx]),
        Action::ChestOpen => (PyActionType::ChestOpen, vec![]),
        Action::PotionUse {
            idx_potion,
            idx_monster: None,
        } => (PyActionType::PotionUse, vec![idx_potion]),
        Action::PotionUse {
            idx_potion,
            idx_monster: Some(m),
        } => (PyActionType::PotionUse, vec![idx_potion, m]),
        Action::PotionDiscard { idx } => (PyActionType::PotionDiscard, vec![idx]),
        Action::CardDiscover { idx } => (PyActionType::CardDiscover, vec![idx]),
        Action::RewardTakeCard { idx } => (PyActionType::RewardTakeCard, vec![idx]),
        Action::RewardTakeRelic => (PyActionType::RewardTakeRelic, vec![]),
        Action::RewardTakePotion { idx } => (PyActionType::RewardTakePotion, vec![idx]),
        Action::RewardTakeGold => (PyActionType::RewardTakeGold, vec![]),
        Action::EventOptionSelect { idx } => (PyActionType::EventOptionSelect, vec![idx]),
        Action::CardPurge { idx } => (PyActionType::CardPurge, vec![idx]),
        Action::CardUpgrade { idx } => (PyActionType::CardUpgrade, vec![idx]),
        Action::CardDuplicate { idx } => (PyActionType::CardDuplicate, vec![idx]),
        Action::CardTransform { idx } => (PyActionType::CardTransform, vec![idx]),
    };
    PyAction {
        action_type,
        idxs,
        kind: None,
    }
}

// Mirrors only EffectKind variants reachable from static card/monster defs; snapshot_effect panics on runtime-only variants
#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamagePhysical",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamagePhysical {
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamagePhysicalIfPoisoned",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamagePhysicalIfPoisoned {
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectHeelHookProc",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectHeelHookProc {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEscapePlanCheck",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEscapePlanCheck {
    pub block: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectGlassKnifeDecay",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectGlassKnifeDecay {
    pub delta: i16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardSetupPick",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardSetupPick {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardNightmarePick",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardNightmarePick {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDistractionAdd",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDistractionAdd {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectSetCostOverride",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectSetCostOverride {
    pub amount: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamageFinisher",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamageFinisher {
    pub damage: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamageFlechettes",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamageFlechettes {
    pub damage: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectUnloadDiscard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectUnloadDiscard {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectStormOfSteelProc",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectStormOfSteelProc {
    pub upgraded: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectSneakyStrikeProc",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectSneakyStrikeProc {
    pub energy: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectBlockGain",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectBlockGain {
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectModifierGain",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectModifierGain {
    pub kind: PyModifierKind,
    pub stacks: i16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectModifierMultiply",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectModifierMultiply {
    pub kind: PyModifierKind,
    pub factor: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectModifierRemove",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectModifierRemove {
    pub kind: PyModifierKind,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEnergyGain",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEnergyGain {
    pub amount: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardAddToHand",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardAddToHand {
    pub card_name: String,
    pub count: u16,
    pub upgraded: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDraw",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDraw {
    pub count: u16,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDrawUpTo",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDrawUpTo {
    pub amount: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDiscard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDiscard {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardRetain",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardRetain {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectDamageMindBlast",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectDamageMindBlast {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectShuffleDiscardPileIntoDrawPile",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectShuffleDiscardPileIntoDrawPile {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCalculatedGamble",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCalculatedGamble {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectMaxHealthDelta",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectMaxHealthDelta {
    pub sign: PyDeltaSign,
    pub amount: PyAmount,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectHealthDelta",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectHealthDelta {
    pub sign: PyDeltaSign,
    pub amount: PyAmount,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectPotionAddRandom",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectPotionAddRandom {
    pub limited: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectPotionDiscard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectPotionDiscard {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectRewardRollPotions",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectRewardRollPotions {
    pub count: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDiscoverRoll",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDiscoverRoll {
    pub kind: PyCardKind,
    pub count: u8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectGoldDelta",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectGoldDelta {
    pub sign: PyDeltaSign,
    pub amount: PyAmount,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectRelicGrantRandom",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectRelicGrantRandom {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectWheelSpin",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectWheelSpin {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectBonfireOffer",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectBonfireOffer {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectFaceTrade",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectFaceTrade {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectMonsterSpawn",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectMonsterSpawn {
    pub name: PyMonsterName,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCombatStart",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCombatStart {
    pub event_gold: Option<PyAmount>,
    pub event_relic: Option<PyRelicName>,
    pub event_relic_roll: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectAdventurerSearch",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectAdventurerSearch {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectRelicGrantSpecific",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectRelicGrantSpecific {
    pub name: PyRelicName,
    pub fallback_circlet: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEventAdvanceState",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEventAdvanceState {
    pub delta: i8,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectScrapOozeReach",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectScrapOozeReach {
    pub dmg: u16,
    pub chance: u8,
    pub advance_on_miss: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectEventConsume",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectEventConsume {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDiscoverPick",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDiscoverPick {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardAddToDeck",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardAddToDeck {
    pub card_name: String,
    pub upgraded: bool,
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardPurge",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardPurge {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardUpgrade",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardUpgrade {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardDuplicate",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardDuplicate {
    pub target: Option<PyTarget>,
}

#[pyclass(
    eq,
    hash,
    frozen,
    get_all,
    name = "EffectCardTransform",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEffectCardTransform {
    pub target: Option<PyTarget>,
}

// NB: variant order matches the old complex enum — card_identity_hash depends on it
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyEffect {
    DamagePhysical(PyEffectDamagePhysical),
    DamagePhysicalIfPoisoned(PyEffectDamagePhysicalIfPoisoned),
    HeelHookProc(PyEffectHeelHookProc),
    EscapePlanCheck(PyEffectEscapePlanCheck),
    GlassKnifeDecay(PyEffectGlassKnifeDecay),
    CardSetupPick(PyEffectCardSetupPick),
    CardNightmarePick(PyEffectCardNightmarePick),
    DistractionAdd(PyEffectDistractionAdd),
    SetCostOverride(PyEffectSetCostOverride),
    DamageFinisher(PyEffectDamageFinisher),
    DamageFlechettes(PyEffectDamageFlechettes),
    UnloadDiscard(PyEffectUnloadDiscard),
    StormOfSteelProc(PyEffectStormOfSteelProc),
    SneakyStrikeProc(PyEffectSneakyStrikeProc),
    BlockGain(PyEffectBlockGain),
    ModifierGain(PyEffectModifierGain),
    ModifierMultiply(PyEffectModifierMultiply),
    ModifierRemove(PyEffectModifierRemove),
    EnergyGain(PyEffectEnergyGain),
    CardAddToHand(PyEffectCardAddToHand),
    CardDraw(PyEffectCardDraw),
    CardDrawUpTo(PyEffectCardDrawUpTo),
    CardDiscard(PyEffectCardDiscard),
    CardRetain(PyEffectCardRetain),
    DamageMindBlast(PyEffectDamageMindBlast),
    ShuffleDiscardPileIntoDrawPile(PyEffectShuffleDiscardPileIntoDrawPile),
    CalculatedGamble(PyEffectCalculatedGamble),
    MaxHealthDelta(PyEffectMaxHealthDelta),
    HealthDelta(PyEffectHealthDelta),
    PotionAddRandom(PyEffectPotionAddRandom),
    PotionDiscard(PyEffectPotionDiscard),
    RewardRollPotions(PyEffectRewardRollPotions),
    CardDiscoverRoll(PyEffectCardDiscoverRoll),
    GoldDelta(PyEffectGoldDelta),
    RelicGrantRandom(PyEffectRelicGrantRandom),
    WheelSpin(PyEffectWheelSpin),
    BonfireOffer(PyEffectBonfireOffer),
    FaceTrade(PyEffectFaceTrade),
    MonsterSpawn(PyEffectMonsterSpawn),
    CombatStart(PyEffectCombatStart),
    AdventurerSearch(PyEffectAdventurerSearch),
    RelicGrantSpecific(PyEffectRelicGrantSpecific),
    EventAdvanceState(PyEffectEventAdvanceState),
    ScrapOozeReach(PyEffectScrapOozeReach),
    EventConsume(PyEffectEventConsume),
    CardDiscoverPick(PyEffectCardDiscoverPick),
    CardAddToDeck(PyEffectCardAddToDeck),
    CardPurge(PyEffectCardPurge),
    CardUpgrade(PyEffectCardUpgrade),
    CardDuplicate(PyEffectCardDuplicate),
    CardTransform(PyEffectCardTransform),
}

variant_union!(PyEffect {
    DamagePhysical => PyEffectDamagePhysical,
    DamagePhysicalIfPoisoned => PyEffectDamagePhysicalIfPoisoned,
    HeelHookProc => PyEffectHeelHookProc,
    EscapePlanCheck => PyEffectEscapePlanCheck,
    GlassKnifeDecay => PyEffectGlassKnifeDecay,
    CardSetupPick => PyEffectCardSetupPick,
    CardNightmarePick => PyEffectCardNightmarePick,
    DistractionAdd => PyEffectDistractionAdd,
    SetCostOverride => PyEffectSetCostOverride,
    DamageFinisher => PyEffectDamageFinisher,
    DamageFlechettes => PyEffectDamageFlechettes,
    UnloadDiscard => PyEffectUnloadDiscard,
    StormOfSteelProc => PyEffectStormOfSteelProc,
    SneakyStrikeProc => PyEffectSneakyStrikeProc,
    BlockGain => PyEffectBlockGain,
    ModifierGain => PyEffectModifierGain,
    ModifierMultiply => PyEffectModifierMultiply,
    ModifierRemove => PyEffectModifierRemove,
    EnergyGain => PyEffectEnergyGain,
    CardAddToHand => PyEffectCardAddToHand,
    CardDraw => PyEffectCardDraw,
    CardDrawUpTo => PyEffectCardDrawUpTo,
    CardDiscard => PyEffectCardDiscard,
    CardRetain => PyEffectCardRetain,
    DamageMindBlast => PyEffectDamageMindBlast,
    ShuffleDiscardPileIntoDrawPile => PyEffectShuffleDiscardPileIntoDrawPile,
    CalculatedGamble => PyEffectCalculatedGamble,
    MaxHealthDelta => PyEffectMaxHealthDelta,
    HealthDelta => PyEffectHealthDelta,
    PotionAddRandom => PyEffectPotionAddRandom,
    PotionDiscard => PyEffectPotionDiscard,
    RewardRollPotions => PyEffectRewardRollPotions,
    CardDiscoverRoll => PyEffectCardDiscoverRoll,
    GoldDelta => PyEffectGoldDelta,
    RelicGrantRandom => PyEffectRelicGrantRandom,
    WheelSpin => PyEffectWheelSpin,
    BonfireOffer => PyEffectBonfireOffer,
    FaceTrade => PyEffectFaceTrade,
    MonsterSpawn => PyEffectMonsterSpawn,
    CombatStart => PyEffectCombatStart,
    AdventurerSearch => PyEffectAdventurerSearch,
    RelicGrantSpecific => PyEffectRelicGrantSpecific,
    EventAdvanceState => PyEffectEventAdvanceState,
    ScrapOozeReach => PyEffectScrapOozeReach,
    EventConsume => PyEffectEventConsume,
    CardDiscoverPick => PyEffectCardDiscoverPick,
    CardAddToDeck => PyEffectCardAddToDeck,
    CardPurge => PyEffectCardPurge,
    CardUpgrade => PyEffectCardUpgrade,
    CardDuplicate => PyEffectCardDuplicate,
    CardTransform => PyEffectCardTransform,
});

fn snapshot_effect(effect: &Effect) -> PyEffect {
    let target = match effect.target {
        Target::Resolve {
            candidate_pool,
            selection_kind,
        } => Some(PyTarget {
            candidate_pool: candidate_pool.into(),
            selection_kind: selection_kind.into(),
        }),
        Target::Direct(None) => None,
        Target::Direct(Some(_)) => panic!(
            "snapshot_effect: unexpected Direct(Some) on static card effect: {:?}",
            effect,
        ),
    };
    match effect.kind {
        EffectKind::DamagePhysical { amount } => {
            PyEffect::DamagePhysical(PyEffectDamagePhysical { amount, target })
        }
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            PyEffect::DamagePhysicalIfPoisoned(PyEffectDamagePhysicalIfPoisoned { amount, target })
        }
        EffectKind::HeelHookProc => PyEffect::HeelHookProc(PyEffectHeelHookProc { target }),
        EffectKind::EscapePlanCheck { block } => {
            PyEffect::EscapePlanCheck(PyEffectEscapePlanCheck { block, target })
        }
        EffectKind::GlassKnifeDecay { delta } => {
            PyEffect::GlassKnifeDecay(PyEffectGlassKnifeDecay { delta, target })
        }
        EffectKind::CardSetupPick => PyEffect::CardSetupPick(PyEffectCardSetupPick { target }),
        EffectKind::CardNightmarePick => {
            PyEffect::CardNightmarePick(PyEffectCardNightmarePick { target })
        }
        EffectKind::DistractionAdd => PyEffect::DistractionAdd(PyEffectDistractionAdd { target }),
        EffectKind::SetCostOverride { amount } => {
            PyEffect::SetCostOverride(PyEffectSetCostOverride { amount, target })
        }
        EffectKind::DamageFinisher { damage } => {
            PyEffect::DamageFinisher(PyEffectDamageFinisher { damage, target })
        }
        EffectKind::DamageFlechettes { damage } => {
            PyEffect::DamageFlechettes(PyEffectDamageFlechettes { damage, target })
        }
        EffectKind::UnloadDiscard => PyEffect::UnloadDiscard(PyEffectUnloadDiscard { target }),
        EffectKind::StormOfSteelProc { upgraded } => {
            PyEffect::StormOfSteelProc(PyEffectStormOfSteelProc { upgraded, target })
        }
        EffectKind::SneakyStrikeProc { energy } => {
            PyEffect::SneakyStrikeProc(PyEffectSneakyStrikeProc { energy, target })
        }
        EffectKind::BlockGain { amount } => {
            PyEffect::BlockGain(PyEffectBlockGain { amount, target })
        }
        EffectKind::ModifierGain { kind, stacks } => PyEffect::ModifierGain(PyEffectModifierGain {
            kind: kind.into(),
            stacks,
            target,
        }),
        EffectKind::ModifierMultiply { kind, factor } => {
            PyEffect::ModifierMultiply(PyEffectModifierMultiply {
                kind: kind.into(),
                factor,
                target,
            })
        }
        EffectKind::ModifierRemove { kind } => PyEffect::ModifierRemove(PyEffectModifierRemove {
            kind: kind.into(),
            target,
        }),
        EffectKind::EnergyGain { amount } => {
            PyEffect::EnergyGain(PyEffectEnergyGain { amount, target })
        }
        EffectKind::CardAddToHand {
            card_name,
            count,
            upgraded,
        } => PyEffect::CardAddToHand(PyEffectCardAddToHand {
            card_name: card_name.as_str().to_string(),
            count,
            upgraded,
            target,
        }),
        EffectKind::CardDraw { count } => PyEffect::CardDraw(PyEffectCardDraw { count, target }),
        EffectKind::CardDrawUpTo { amount } => {
            PyEffect::CardDrawUpTo(PyEffectCardDrawUpTo { amount, target })
        }
        EffectKind::CardDiscard { source: _ } => {
            PyEffect::CardDiscard(PyEffectCardDiscard { target })
        }
        EffectKind::CardRetain => PyEffect::CardRetain(PyEffectCardRetain { target }),
        EffectKind::DamageMindBlast => {
            PyEffect::DamageMindBlast(PyEffectDamageMindBlast { target })
        }
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            PyEffect::ShuffleDiscardPileIntoDrawPile(PyEffectShuffleDiscardPileIntoDrawPile {
                target,
            })
        }
        EffectKind::CalculatedGamble => {
            PyEffect::CalculatedGamble(PyEffectCalculatedGamble { target })
        }
        EffectKind::GoldDelta { sign, amount } => PyEffect::GoldDelta(PyEffectGoldDelta {
            sign: sign.into(),
            amount: amount.into(),
            target,
        }),
        EffectKind::HealthDelta { sign, amount } => PyEffect::HealthDelta(PyEffectHealthDelta {
            sign: sign.into(),
            amount: amount.into(),
            target,
        }),
        EffectKind::MaxHealthDelta { sign, amount } => {
            PyEffect::MaxHealthDelta(PyEffectMaxHealthDelta {
                sign: sign.into(),
                amount: amount.into(),
                target,
            })
        }
        EffectKind::CardPurge => PyEffect::CardPurge(PyEffectCardPurge { target }),
        EffectKind::CardDuplicate => PyEffect::CardDuplicate(PyEffectCardDuplicate { target }),
        EffectKind::CardTransform => PyEffect::CardTransform(PyEffectCardTransform { target }),
        EffectKind::RelicGrantRandom => {
            PyEffect::RelicGrantRandom(PyEffectRelicGrantRandom { target })
        }
        EffectKind::WheelSpin => PyEffect::WheelSpin(PyEffectWheelSpin { target }),
        EffectKind::BonfireOffer => PyEffect::BonfireOffer(PyEffectBonfireOffer { target }),
        EffectKind::FaceTrade => PyEffect::FaceTrade(PyEffectFaceTrade { target }),
        EffectKind::MonsterSpawn { name } => PyEffect::MonsterSpawn(PyEffectMonsterSpawn {
            name: name.into(),
            target,
        }),
        EffectKind::CombatStart {
            event_gold,
            event_relic,
            event_relic_roll,
        } => PyEffect::CombatStart(PyEffectCombatStart {
            event_gold: event_gold.map(Into::into),
            event_relic: event_relic.map(Into::into),
            event_relic_roll,
            target,
        }),
        EffectKind::AdventurerSearch => {
            PyEffect::AdventurerSearch(PyEffectAdventurerSearch { target })
        }
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => PyEffect::RelicGrantSpecific(PyEffectRelicGrantSpecific {
            name: name.into(),
            fallback_circlet,
            target,
        }),
        EffectKind::EventAdvanceState { delta } => {
            PyEffect::EventAdvanceState(PyEffectEventAdvanceState { delta, target })
        }
        EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        } => PyEffect::ScrapOozeReach(PyEffectScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
            target,
        }),
        EffectKind::EventConsume => PyEffect::EventConsume(PyEffectEventConsume { target }),
        EffectKind::CardAddToDeck {
            card_name,
            upgraded,
        } => PyEffect::CardAddToDeck(PyEffectCardAddToDeck {
            card_name: card_name.as_str().to_string(),
            upgraded,
            target,
        }),
        EffectKind::PotionAddRandom { limited } => {
            PyEffect::PotionAddRandom(PyEffectPotionAddRandom { limited, target })
        }
        EffectKind::PotionDiscard => PyEffect::PotionDiscard(PyEffectPotionDiscard { target }),
        EffectKind::RewardRollPotions { count } => {
            PyEffect::RewardRollPotions(PyEffectRewardRollPotions { count, target })
        }
        EffectKind::CardDiscoverRoll { kind, count } => {
            PyEffect::CardDiscoverRoll(PyEffectCardDiscoverRoll {
                kind: kind.into(),
                count,
                target,
            })
        }
        EffectKind::CardUpgrade => PyEffect::CardUpgrade(PyEffectCardUpgrade { target }),
        EffectKind::CardDiscoverPick => {
            PyEffect::CardDiscoverPick(PyEffectCardDiscoverPick { target })
        }
        other => unreachable!(
            "snapshot_effect: unexpected EffectKind on static card effect: {:?}",
            other
        ),
    }
}

// Exposed structs
#[pyclass(frozen, get_all, name = "Card", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyCard {
    pub name: PyCardName,
    pub display_name: String,

    // Cost-related fields
    pub cost: u8,
    pub cost_base: u8,
    pub cost_zero_once: bool,
    pub cost_override: Option<u8>,
    pub cost_kind: PyCardCostKind,

    // Categorical fields
    pub kind: PyCardKind,
    pub color: PyCardColor,
    pub rarity: PyCardRarity,
    pub play_restriction: PyPlayRestriction,

    // Other boolean fields
    pub upgraded: bool,
    pub exhaust: bool,
    pub ethereal: bool,
    pub innate: bool,
    pub requires_target: bool,
    pub retain: bool,
    // `playable` does NOT factor in energy cost; clients must also check `cost <= energy.energy_current`
    pub playable: bool,

    // Effects. Snapshot copy: DamagePhysical / BlockGain amounts carry the current player-modifier
    // adjustment (Str/Vigor/Weak/DoubleDamage, Dex/Frail), target-agnostic, so clients read finished
    // combat values. This makes identity_hash (which hashes effects) vary with combat modifiers.
    pub effects: Vec<PyEffect>,

    // Fingerprint over every snapshot field above except display_name (derived from
    // name+upgraded): one u64 getter replaces a per-field FFI walk for clients that
    // key caches/dedup on card identity. Deterministic across processes.
    pub identity_hash: u64,
}

#[pyclass(frozen, get_all, name = "Modifier", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModifier {
    pub kind: PyModifierKind,
    pub stacks: i16,
    pub stacks_max: i16,
}

#[pymethods]
impl PyModifier {
    #[new]
    fn new(kind: PyModifierKind, stacks: i16, stacks_max: i16) -> Self {
        Self {
            kind,
            stacks,
            stacks_max,
        }
    }
}

#[pyclass(frozen, get_all, name = "Relic", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyRelic {
    pub name: PyRelicName,
    pub tier: PyRelicTier,
    pub counter: i16,
    pub used_up: bool,
    pub effects_on_combat_start: Vec<PyEffect>,
}

#[pyclass(frozen, get_all, name = "Potion", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyPotion {
    pub name: PyPotionName,
    pub rarity: PyPotionRarity,
    pub requires_target: bool,
    pub combat_only: bool,
    pub effects: Vec<PyEffect>,
}

#[pyclass(frozen, name = "EventKindBigFish", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindBigFish;

#[pyclass(frozen, name = "EventKindTheCleric", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindTheCleric;

#[pyclass(frozen, name = "EventKindDuplicator", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindDuplicator;

#[pyclass(frozen, name = "EventKindGoldenShrine", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindGoldenShrine;

#[pyclass(frozen, name = "EventKindWingStatue", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindWingStatue;

#[pyclass(frozen, name = "EventKindWorldOfGoop", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindWorldOfGoop;

#[pyclass(frozen, name = "EventKindLivingWall", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindLivingWall;

#[pyclass(frozen, name = "EventKindPurifier", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindPurifier;

#[pyclass(frozen, name = "EventKindShiningLight", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindShiningLight;

#[pyclass(frozen, name = "EventKindTheSsssserpent", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindTheSsssserpent;

#[pyclass(frozen, name = "EventKindTransmogrifier", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindTransmogrifier;

#[pyclass(frozen, name = "EventKindUpgradeShrine", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindUpgradeShrine;

#[pyclass(frozen, name = "EventKindTheDivineFountain", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindTheDivineFountain;

#[pyclass(frozen, name = "EventKindTheLab", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindTheLab;

#[pyclass(frozen, name = "EventKindTheWomanInBlue", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindTheWomanInBlue;

#[pyclass(frozen, name = "EventKindWheelOfChange", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindWheelOfChange;

#[pyclass(frozen, name = "EventKindBonfireSpirits", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindBonfireSpirits;

#[pyclass(frozen, name = "EventKindOminousForge", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindOminousForge;

#[pyclass(frozen, name = "EventKindFaceTrader", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindFaceTrader;

#[pyclass(frozen, name = "EventKindMushrooms", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindMushrooms;

#[pyclass(frozen, get_all, name = "EventKindGoldenIdol", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindGoldenIdol {
    pub stage: u8,
}

#[pyclass(frozen, get_all, name = "EventKindScrapOoze", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindScrapOoze {
    pub attempts: u8,
}

#[pyclass(frozen, get_all, name = "EventKindWeMeetAgain", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventKindWeMeetAgain {
    pub pick_card: Option<PyCard>,
    pub pick_potion: Option<PyPotion>,
    pub gold_ask: Option<u16>,
}

#[pyclass(
    frozen,
    get_all,
    name = "EventKindDeadAdventurer",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyEventKindDeadAdventurer {
    pub found_gold: bool,
    pub found_nothing: bool,
    pub found_relic: bool,
    pub searches: u8,
}

#[derive(Debug, Clone)]
pub enum PyEventKind {
    BigFish(PyEventKindBigFish),
    TheCleric(PyEventKindTheCleric),
    Duplicator(PyEventKindDuplicator),
    GoldenShrine(PyEventKindGoldenShrine),
    WingStatue(PyEventKindWingStatue),
    WorldOfGoop(PyEventKindWorldOfGoop),
    LivingWall(PyEventKindLivingWall),
    Purifier(PyEventKindPurifier),
    ShiningLight(PyEventKindShiningLight),
    TheSsssserpent(PyEventKindTheSsssserpent),
    Transmogrifier(PyEventKindTransmogrifier),
    UpgradeShrine(PyEventKindUpgradeShrine),
    TheDivineFountain(PyEventKindTheDivineFountain),
    TheLab(PyEventKindTheLab),
    TheWomanInBlue(PyEventKindTheWomanInBlue),
    WheelOfChange(PyEventKindWheelOfChange),
    BonfireSpirits(PyEventKindBonfireSpirits),
    OminousForge(PyEventKindOminousForge),
    FaceTrader(PyEventKindFaceTrader),
    Mushrooms(PyEventKindMushrooms),
    GoldenIdol(PyEventKindGoldenIdol),
    ScrapOoze(PyEventKindScrapOoze),
    WeMeetAgain(PyEventKindWeMeetAgain),
    DeadAdventurer(PyEventKindDeadAdventurer),
}

variant_union!(PyEventKind {
    BigFish => PyEventKindBigFish,
    TheCleric => PyEventKindTheCleric,
    Duplicator => PyEventKindDuplicator,
    GoldenShrine => PyEventKindGoldenShrine,
    WingStatue => PyEventKindWingStatue,
    WorldOfGoop => PyEventKindWorldOfGoop,
    LivingWall => PyEventKindLivingWall,
    Purifier => PyEventKindPurifier,
    ShiningLight => PyEventKindShiningLight,
    TheSsssserpent => PyEventKindTheSsssserpent,
    Transmogrifier => PyEventKindTransmogrifier,
    UpgradeShrine => PyEventKindUpgradeShrine,
    TheDivineFountain => PyEventKindTheDivineFountain,
    TheLab => PyEventKindTheLab,
    TheWomanInBlue => PyEventKindTheWomanInBlue,
    WheelOfChange => PyEventKindWheelOfChange,
    BonfireSpirits => PyEventKindBonfireSpirits,
    OminousForge => PyEventKindOminousForge,
    FaceTrader => PyEventKindFaceTrader,
    Mushrooms => PyEventKindMushrooms,
    GoldenIdol => PyEventKindGoldenIdol,
    ScrapOoze => PyEventKindScrapOoze,
    WeMeetAgain => PyEventKindWeMeetAgain,
    DeadAdventurer => PyEventKindDeadAdventurer,
});

#[pyclass(frozen, name = "ModeMap", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeMap;

#[pyclass(frozen, name = "ModeRestSite", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeRestSite;

#[pyclass(frozen, name = "ModeChest", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeChest;

#[pyclass(frozen, name = "ModeChestOpened", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeChestOpened;

#[pyclass(frozen, name = "ModeCombatEnded", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeCombatEnded;

#[pyclass(frozen, get_all, name = "ModeCombat", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeCombat {
    pub hand: Vec<PyCard>,
    pub pile_draw: Vec<PyCard>,
    pub pile_discard: Vec<PyCard>,
    pub pile_exhaust: Vec<PyCard>,
    pub energy: PyEnergy,
    pub monsters: Vec<PyMonster>,
    pub discover: Vec<PyCard>,
}

#[pyclass(frozen, get_all, name = "ModeReward", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeReward {
    pub cards: Vec<PyCard>,
    pub relic: Option<PyRelic>,
    pub potions: Vec<PyPotion>,
    pub gold: Option<u16>,
}

#[pyclass(frozen, get_all, name = "ModeShop", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeShop {
    pub cards: Vec<PyCard>,
    pub card_prices: Vec<u16>,
    pub relics: Vec<PyRelic>,
    pub relic_prices: Vec<u16>,
    pub potions: Vec<PyPotion>,
    pub potion_prices: Vec<u16>,
    pub purge_cost: u16,
}

#[pyclass(frozen, get_all, name = "ModeEvent", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeEvent {
    pub kind: PyEventKind,
    pub options: Vec<Vec<PyEffect>>,
    pub consumed: bool,
}

#[derive(Debug, Clone)]
pub enum PyMode {
    Map(PyModeMap),
    RestSite(PyModeRestSite),
    Chest(PyModeChest),
    ChestOpened(PyModeChestOpened),
    CombatEnded(PyModeCombatEnded),
    Combat(PyModeCombat),
    Reward(PyModeReward),
    Shop(PyModeShop),
    Event(PyModeEvent),
}

variant_union!(PyMode {
    Map => PyModeMap,
    RestSite => PyModeRestSite,
    Chest => PyModeChest,
    ChestOpened => PyModeChestOpened,
    CombatEnded => PyModeCombatEnded,
    Combat => PyModeCombat,
    Reward => PyModeReward,
    Shop => PyModeShop,
    Event => PyModeEvent,
});

#[pyclass(frozen, get_all, name = "Character", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyCharacter {
    pub name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub gold: u16,
}

#[pymethods]
impl PyCharacter {
    #[new]
    fn new(
        name: String,
        health: u16,
        health_max: u16,
        block: u16,
        modifiers: Vec<PyModifier>,
        gold: u16,
    ) -> Self {
        Self {
            name,
            health,
            health_max,
            block,
            modifiers,
            gold,
        }
    }
}

#[pyclass(eq, eq_int, frozen, name = "IntentKind", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyIntentKind {
    Attack,
    AttackBlock,
    AttackBuff,
    AttackDebuff,
    Block,
    BlockBuff,
    Buff,
    Debuff,
    DebuffPowerful,
    Escape,
    Sleep,
    Stunned,
    Unknown,
}

impl From<Intent> for PyIntentKind {
    fn from(intent: Intent) -> Self {
        match intent {
            Intent::Attack { .. } => Self::Attack,
            Intent::AttackBlock { .. } => Self::AttackBlock,
            Intent::AttackBuff { .. } => Self::AttackBuff,
            Intent::AttackDebuff { .. } => Self::AttackDebuff,
            Intent::Block => Self::Block,
            Intent::BlockBuff => Self::BlockBuff,
            Intent::Buff => Self::Buff,
            Intent::Debuff => Self::Debuff,
            Intent::DebuffPowerful => Self::DebuffPowerful,
            Intent::Escape => Self::Escape,
            Intent::Sleep => Self::Sleep,
            Intent::Stunned => Self::Stunned,
            Intent::Unknown => Self::Unknown,
        }
    }
}

#[pyclass(frozen, get_all, name = "Intent", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyIntent {
    pub kind: PyIntentKind,
    pub damage: Option<u16>,
    pub instances: Option<u8>,
}

#[pymethods]
impl PyIntent {
    #[new]
    fn new(kind: PyIntentKind, damage: Option<u16>, instances: Option<u8>) -> Self {
        Self {
            kind,
            damage,
            instances,
        }
    }
}

#[pyclass(frozen, get_all, name = "Monster", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyMonster {
    pub name: PyMonsterName,
    pub display_name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub intent: PyIntent,
}

#[pymethods]
impl PyMonster {
    #[new]
    fn new(
        name: PyMonsterName,
        display_name: String,
        health: u16,
        health_max: u16,
        block: u16,
        modifiers: Vec<PyModifier>,
        intent: PyIntent,
    ) -> Self {
        Self {
            name,
            display_name,
            health,
            health_max,
            block,
            modifiers,
            intent,
        }
    }
}

#[pyclass(frozen, get_all, name = "Energy", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEnergy {
    pub energy_current: u8,
    pub energy_max: u8,
}

#[pymethods]
impl PyEnergy {
    #[new]
    fn new(energy_current: u8, energy_max: u8) -> Self {
        Self {
            energy_current,
            energy_max,
        }
    }
}

#[pyclass(frozen, get_all, name = "Room", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyRoom {
    pub room_kind: PyRoomKind,
    pub edges: Vec<usize>,
    pub chest_opened: bool,
}

#[pymethods]
impl PyRoom {
    #[new]
    fn new(room_kind: PyRoomKind, edges: Vec<usize>, chest_opened: bool) -> Self {
        Self {
            room_kind,
            edges,
            chest_opened,
        }
    }
}

#[pyclass(frozen, get_all, name = "Map", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyMap {
    pub rooms: Vec<Vec<Option<PyRoom>>>,
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
    pub boss: PyMonsterEncounter,
    pub identity_hash: u64,
}

#[pymethods]
impl PyMap {
    #[new]
    fn new(
        rooms: Vec<Vec<Option<PyRoom>>>,
        y_current: Option<usize>,
        x_current: Option<usize>,
        boss: PyMonsterEncounter,
        identity_hash: u64,
    ) -> Self {
        Self {
            rooms,
            y_current,
            x_current,
            boss,
            identity_hash,
        }
    }
}

#[pyclass(frozen, get_all, name = "GameState", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyGameState {
    pub mode: PyMode,
    pub game_over: bool,
    pub ascension: u8,
    pub character: PyCharacter,
    pub deck: Vec<PyCard>,
    pub relics: Vec<PyRelic>,
    // Slot-indexed belt (length potion_slots_max); None at empty slots so positions stay valid
    pub potions: Vec<Option<PyPotion>>,
    pub potion_slots_max: u8,
    pub map: PyMap,
    // Halt-for-input is orthogonal to mode
    pub pending: Option<PyEffect>,
}

// Display-name lookups
impl CardName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AThousandCuts => "A Thousand Cuts",
            Self::Accuracy => "Accuracy",
            Self::Acrobatics => "Acrobatics",
            Self::Adrenaline => "Adrenaline",
            Self::AfterImage => "After Image",
            Self::Alchemize => "Alchemize",
            Self::AllOutAttack => "All Out Attack",
            Self::Backflip => "Backflip",
            Self::Backstab => "Backstab",
            Self::BandageUp => "Bandage Up",
            Self::Bane => "Bane",
            Self::BladeDance => "Blade Dance",
            Self::Blind => "Blind",
            Self::Blur => "Blur",
            Self::BouncingFlask => "Bouncing Flask",
            Self::BulletTime => "Bullet Time",
            Self::Burn => "Burn",
            Self::Burst => "Burst",
            Self::CalculatedGamble => "Calculated Gamble",
            Self::Caltrops => "Caltrops",
            Self::Catalyst => "Catalyst",
            Self::Choke => "Choke",
            Self::CloakAndDagger => "Cloak And Dagger",
            Self::Concentrate => "Concentrate",
            Self::CorpseExplosion => "Corpse Explosion",
            Self::CripplingPoison => "Crippling Poison",
            Self::DaggerSpray => "Dagger Spray",
            Self::DaggerThrow => "Dagger Throw",
            Self::Dash => "Dash",
            Self::Dazed => "Dazed",
            Self::DeadlyPoison => "Deadly Poison",
            Self::DeepBreath => "Deep Breath",
            Self::Defend => "Defend",
            Self::Deflect => "Deflect",
            Self::DieDieDie => "Die Die Die",
            Self::Distraction => "Distraction",
            Self::DodgeAndRoll => "Dodge And Roll",
            Self::Doppelganger => "Doppelganger",
            Self::EndlessAgony => "Endless Agony",
            Self::Envenom => "Envenom",
            Self::EscapePlan => "Escape Plan",
            Self::Eviscerate => "Eviscerate",
            Self::Expertise => "Expertise",
            Self::Finesse => "Finesse",
            Self::Finisher => "Finisher",
            Self::FlashOfSteel => "Flash Of Steel",
            Self::Flechettes => "Flechettes",
            Self::FlyingKnee => "Flying Knee",
            Self::Footwork => "Footwork",
            Self::GlassKnife => "Glass Knife",
            Self::GoodInstincts => "Good Instincts",
            Self::GrandFinale => "Grand Finale",
            Self::HeelHook => "Heel Hook",
            Self::InfiniteBlades => "Infinite Blades",
            Self::LegSweep => "Leg Sweep",
            Self::Malaise => "Malaise",
            Self::MasterOfStrategy => "Master Of Strategy",
            Self::MasterfulStab => "Masterful Stab",
            Self::MindBlast => "Mind Blast",
            Self::Neutralize => "Neutralize",
            Self::Nightmare => "Nightmare",
            Self::NoxiousFumes => "Noxious Fumes",
            Self::Outmaneuver => "Outmaneuver",
            Self::PhantasmalKiller => "Phantasmal Killer",
            Self::PiercingWail => "Piercing Wail",
            Self::PoisonedStab => "Poisoned Stab",
            Self::Predator => "Predator",
            Self::Prepared => "Prepared",
            Self::QuickSlash => "Quick Slash",
            Self::Reflex => "Reflex",
            Self::RiddleWithHoles => "Riddle With Holes",
            Self::Setup => "Setup",
            Self::Shiv => "Shiv",
            Self::Skewer => "Skewer",
            Self::Slice => "Slice",
            Self::Slimed => "Slimed",
            Self::SneakyStrike => "Sneaky Strike",
            Self::StormOfSteel => "Storm Of Steel",
            Self::Strike => "Strike",
            Self::SuckerPunch => "Sucker Punch",
            Self::Survivor => "Survivor",
            Self::SwiftStrike => "Swift Strike",
            Self::Tactician => "Tactician",
            Self::Terror => "Terror",
            Self::ToolsOfTheTrade => "Tools Of The Trade",
            Self::Unload => "Unload",
            Self::WellLaidPlans => "Well Laid Plans",
            Self::WraithForm => "Wraith Form",
            Self::AscendersBane => "Ascender's Bane",
            Self::Regret => "Regret",
            Self::Pain => "Pain",
            Self::Doubt => "Doubt",
            Self::Decay => "Decay",
            Self::Injury => "Injury",
            Self::Shame => "Shame",
            Self::Writhe => "Writhe",
            Self::Parasite => "Parasite",
            Self::Normality => "Normality",
        }
    }
}

impl MonsterName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cultist => "Cultist",
            Self::FungiBeast => "Fungi Beast",
            Self::GremlinFat => "Fat Gremlin",
            Self::GremlinNob => "Gremlin Nob",
            Self::GremlinThief => "Sneaky Gremlin",
            Self::GremlinTsundere => "Shield Gremlin",
            Self::GremlinWarrior => "Mad Gremlin",
            Self::GremlinWizard => "Gremlin Wizard",
            Self::Hexaghost => "Hexaghost",
            Self::JawWorm => "Jaw Worm",
            Self::Lagavulin => "Lagavulin",
            Self::Looter => "Looter",
            Self::LouseDefensive => "Green Louse",
            Self::LouseNormal => "Red Louse",
            Self::Sentry => "Sentry",
            Self::SlaverBlue => "Blue Slaver",
            Self::SlaverRed => "Red Slaver",
            Self::SlimeAcidLarge => "Acid Slime (L)",
            Self::SlimeAcidMedium => "Acid Slime (M)",
            Self::SlimeAcidSmall => "Acid Slime (S)",
            Self::SlimeBoss => "Slime Boss",
            Self::SlimeSpikeLarge => "Spike Slime (L)",
            Self::SlimeSpikeMedium => "Spike Slime (M)",
            Self::SlimeSpikeSmall => "Spike Slime (S)",
            Self::TheGuardian => "The Guardian",
        }
    }
}

impl MonsterEncounter {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cultist => "Cultist",
            Self::JawWorm => "Jaw Worm",
            Self::TwoLouse => "2 Louse",
            Self::SmallSlimes => "Small Slimes",
            Self::BlueSlaver => "Blue Slaver",
            Self::RedSlaver => "Red Slaver",
            Self::Looter => "Looter",
            Self::TwoFungiBeasts => "2 Fungi Beasts",
            Self::ThreeFungiBeasts => "3 Fungi Beasts",
            Self::ThreeLouse => "3 Louse",
            Self::LargeSlime => "Large Slime",
            Self::LotsOfSlimes => "Lots of Slimes",
            Self::GremlinGang => "Gremlin Gang",
            Self::ExordiumThugs => "Exordium Thugs",
            Self::ExordiumWildlife => "Exordium Wildlife",
            Self::GremlinNob => "Gremlin Nob",
            Self::Lagavulin => "Lagavulin",
            Self::ThreeSentries => "3 Sentries",
            Self::TheGuardian => "The Guardian",
            Self::Hexaghost => "Hexaghost",
            Self::SlimeBoss => "Slime Boss",
        }
    }
}

// Snapshot builders
pub fn snapshot_state(state: &GameState) -> PyGameState {
    PyGameState {
        mode: snapshot_mode(state),
        game_over: state.game_over,
        ascension: state.ascension,
        character: snapshot_character(state),
        deck: state
            .id_deck
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        relics: iter_owned_relics(&state.id_relics)
            .map(|(_name, id)| snapshot_relic(&state.entities[id]))
            .collect(),
        potions: state.id_potions[..state.potion_slots_max as usize]
            .iter()
            .map(|s| s.map(|id| snapshot_potion(&state.entities[id])))
            .collect(),
        potion_slots_max: state.potion_slots_max,
        map: snapshot_map(state),
        pending: state.effect_pending.as_ref().map(snapshot_effect),
    }
}

fn snapshot_mode(state: &GameState) -> PyMode {
    match &state.mode {
        Mode::Map => PyMode::Map(PyModeMap),
        Mode::RestSite => PyMode::RestSite(PyModeRestSite),
        Mode::Chest => PyMode::Chest(PyModeChest),
        Mode::ChestOpened => PyMode::ChestOpened(PyModeChestOpened),
        Mode::CombatEnded => PyMode::CombatEnded(PyModeCombatEnded),
        Mode::Combat {
            id_hand,
            id_pile_draw,
            id_pile_discard,
            id_pile_exhaust,
            energy,
            id_discover,
            ..
        } => PyMode::Combat(PyModeCombat {
            hand: id_hand.iter().map(|&id| snapshot_card(state, id)).collect(),
            pile_draw: id_pile_draw
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            pile_discard: id_pile_discard
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            pile_exhaust: id_pile_exhaust
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            energy: PyEnergy {
                energy_current: energy.energy_current,
                energy_max: energy.energy_max,
            },
            monsters: snapshot_monsters(state),
            discover: id_discover
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
        }),
        Mode::Reward {
            reward_id_cards,
            reward_id_relic,
            reward_id_potions,
            reward_gold,
        } => PyMode::Reward(PyModeReward {
            cards: reward_id_cards
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            relic: reward_id_relic.map(|id| snapshot_relic(&state.entities[id])),
            potions: reward_id_potions
                .iter()
                .map(|&id| snapshot_potion(&state.entities[id]))
                .collect(),
            gold: *reward_gold,
        }),
        Mode::Shop {
            shop_id_cards,
            shop_id_relics,
            shop_id_potions,
            shop_card_prices,
            shop_relic_prices,
            shop_potion_prices,
            shop_purge_cost,
        } => PyMode::Shop(PyModeShop {
            cards: shop_id_cards
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            card_prices: shop_card_prices.clone(),
            relics: shop_id_relics
                .iter()
                .map(|&id| snapshot_relic(&state.entities[id]))
                .collect(),
            relic_prices: shop_relic_prices.clone(),
            potions: shop_id_potions
                .iter()
                .map(|&id| snapshot_potion(&state.entities[id]))
                .collect(),
            potion_prices: shop_potion_prices.clone(),
            purge_cost: *shop_purge_cost,
        }),
        Mode::Event {
            kind,
            consumed,
            id_options,
        } => PyMode::Event(PyModeEvent {
            kind: snapshot_event_kind(state, *kind),
            options: id_options
                .iter()
                .map(|&id| {
                    state.entities[id]
                        .event_option_effects
                        .iter()
                        .map(snapshot_effect)
                        .collect()
                })
                .collect(),
            consumed: *consumed,
        }),
    }
}

fn snapshot_event_kind(state: &GameState, kind: EventKind) -> PyEventKind {
    match kind {
        EventKind::BigFish => PyEventKind::BigFish(PyEventKindBigFish),
        EventKind::TheCleric => PyEventKind::TheCleric(PyEventKindTheCleric),
        EventKind::Duplicator => PyEventKind::Duplicator(PyEventKindDuplicator),
        EventKind::GoldenShrine => PyEventKind::GoldenShrine(PyEventKindGoldenShrine),
        EventKind::WingStatue => PyEventKind::WingStatue(PyEventKindWingStatue),
        EventKind::WorldOfGoop => PyEventKind::WorldOfGoop(PyEventKindWorldOfGoop),
        EventKind::LivingWall => PyEventKind::LivingWall(PyEventKindLivingWall),
        EventKind::Purifier => PyEventKind::Purifier(PyEventKindPurifier),
        EventKind::ShiningLight => PyEventKind::ShiningLight(PyEventKindShiningLight),
        EventKind::TheSsssserpent => PyEventKind::TheSsssserpent(PyEventKindTheSsssserpent),
        EventKind::Transmogrifier => PyEventKind::Transmogrifier(PyEventKindTransmogrifier),
        EventKind::UpgradeShrine => PyEventKind::UpgradeShrine(PyEventKindUpgradeShrine),
        EventKind::TheDivineFountain => {
            PyEventKind::TheDivineFountain(PyEventKindTheDivineFountain)
        }
        EventKind::TheLab => PyEventKind::TheLab(PyEventKindTheLab),
        EventKind::TheWomanInBlue => PyEventKind::TheWomanInBlue(PyEventKindTheWomanInBlue),
        EventKind::WheelOfChange => PyEventKind::WheelOfChange(PyEventKindWheelOfChange),
        EventKind::BonfireSpirits => PyEventKind::BonfireSpirits(PyEventKindBonfireSpirits),
        EventKind::OminousForge => PyEventKind::OminousForge(PyEventKindOminousForge),
        EventKind::FaceTrader => PyEventKind::FaceTrader(PyEventKindFaceTrader),
        EventKind::Mushrooms => PyEventKind::Mushrooms(PyEventKindMushrooms),
        EventKind::GoldenIdol { stage } => PyEventKind::GoldenIdol(PyEventKindGoldenIdol { stage }),
        EventKind::ScrapOoze { attempts } => {
            PyEventKind::ScrapOoze(PyEventKindScrapOoze { attempts })
        }
        EventKind::WeMeetAgain {
            id_card,
            id_potion,
            gold_ask,
        } => PyEventKind::WeMeetAgain(PyEventKindWeMeetAgain {
            pick_card: id_card.map(|id| snapshot_card(state, id)),
            pick_potion: id_potion.map(|id| snapshot_potion(&state.entities[id])),
            gold_ask,
        }),
        EventKind::DeadAdventurer {
            found_gold,
            found_nothing,
            found_relic,
            searches,
        } => PyEventKind::DeadAdventurer(PyEventKindDeadAdventurer {
            found_gold,
            found_nothing,
            found_relic,
            searches,
        }),
    }
}

fn snapshot_relic(entity: &Entity) -> PyRelic {
    PyRelic {
        name: entity.relic_name.into(),
        tier: entity.relic_tier.into(),
        counter: entity.relic_counter,
        used_up: entity.relic_used_up,
        effects_on_combat_start: entity
            .relic_effects_on_combat_start
            .iter()
            .map(snapshot_effect)
            .collect(),
    }
}

fn snapshot_potion(entity: &Entity) -> PyPotion {
    PyPotion {
        name: entity.potion_name.into(),
        rarity: entity.potion_rarity.into(),
        requires_target: entity.requires_target,
        combat_only: entity.potion_combat_only,
        effects: entity.potion_effects.iter().map(snapshot_effect).collect(),
    }
}

fn snapshot_character(state: &GameState) -> PyCharacter {
    let character = &state.entities[state.id_character];
    PyCharacter {
        name: character.character_name.to_string(),
        health: character.vitals.health,
        health_max: character.vitals.health_max,
        block: character.vitals.block,
        modifiers: snapshot_modifiers(&character.modifiers),
        gold: character.character_gold,
    }
}

fn snapshot_monsters(state: &GameState) -> Vec<PyMonster> {
    let Mode::Combat { id_monsters, .. } = &state.mode else {
        return Vec::new();
    };
    let character = &state.entities[state.id_character];
    let mods_char = &character.modifiers;
    id_monsters
        .iter()
        .flatten()
        .copied()
        .map(|id_monster| {
            let m = &state.entities[id_monster];

            let intent = if let Some(move_idx) = m.monster_move_current {
                let mv = &m.monster_moves[move_idx];
                let (base_damage, instances) = match mv.intent {
                    Intent::Attack { damage, instances }
                    | Intent::AttackBlock { damage, instances }
                    | Intent::AttackBuff { damage, instances }
                    | Intent::AttackDebuff { damage, instances } => (Some(damage), Some(instances)),
                    Intent::Block
                    | Intent::BlockBuff
                    | Intent::Buff
                    | Intent::Debuff
                    | Intent::DebuffPowerful
                    | Intent::Escape
                    | Intent::Sleep
                    | Intent::Stunned
                    | Intent::Unknown => (None, None),
                };

                let damage = base_damage.map(|d| {
                    let str_stacks = if has_modifier(&m.modifiers, ModifierKind::Strength) {
                        modifier_stacks(&m.modifiers, ModifierKind::Strength)
                    } else {
                        0
                    };
                    let mut scaled = scale_attack_damage(
                        d,
                        str_stacks,
                        weak_factor(
                            has_modifier(&m.modifiers, ModifierKind::Weak),
                            has_relic(&state.id_relics, RelicName::PaperKrane),
                        ),
                        vuln_factor(
                            has_modifier(mods_char, ModifierKind::Vulnerable),
                            has_relic(&state.id_relics, RelicName::OddMushroom),
                        ),
                    );
                    if has_modifier(mods_char, ModifierKind::Intangible) && scaled > 1 {
                        scaled = 1;
                    }
                    scaled
                });

                PyIntent {
                    kind: mv.intent.into(),
                    damage,
                    instances,
                }
            } else {
                PyIntent {
                    kind: PyIntentKind::Unknown,
                    damage: None,
                    instances: None,
                }
            };

            PyMonster {
                name: m.monster_name.into(),
                display_name: m.monster_name.as_str().to_string(),
                health: m.vitals.health,
                health_max: m.vitals.health_max,
                block: m.vitals.block,
                modifiers: snapshot_modifiers(&m.modifiers),
                intent,
            }
        })
        .collect()
}

fn snapshot_modifiers(mods: &Modifiers) -> Vec<PyModifier> {
    active_modifier_kinds(mods.active)
        .map(|kind| PyModifier {
            kind: kind.into(),
            stacks: mods.stacks[kind as usize],
            stacks_max: stacks_max_for(kind),
        })
        .collect()
}

// Snapshot a card's effects with the current player modifiers folded into the DamagePhysical /
// BlockGain amounts (target-agnostic — Vulnerable/Intangible depend on the L3 target chosen later),
// via the same scaling utils as the live pipeline. Other effect kinds pass through unchanged.
fn snapshot_adjusted_effects(card: &Entity, char_mods: &Modifiers) -> Vec<PyEffect> {
    let vigor = if has_modifier(char_mods, ModifierKind::Vigor) {
        modifier_stacks(char_mods, ModifierKind::Vigor).max(0) as u16
    } else {
        0
    };
    let str_stacks = if has_modifier(char_mods, ModifierKind::Strength) {
        modifier_stacks(char_mods, ModifierKind::Strength)
    } else {
        0
    };
    let weak = has_modifier(char_mods, ModifierKind::Weak);
    let double = has_modifier(char_mods, ModifierKind::DoubleDamage);
    let dex = if has_modifier(char_mods, ModifierKind::Dexterity) {
        modifier_stacks(char_mods, ModifierKind::Dexterity)
    } else {
        0
    };
    let frail = has_modifier(char_mods, ModifierKind::Frail);

    card.card_effects[..card.card_effects_len as usize]
        .iter()
        .map(snapshot_effect)
        .map(|effect| match effect {
            PyEffect::DamagePhysical(PyEffectDamagePhysical { amount, target }) => {
                // Player attacker: Paper Krane never applies
                let mut d = scale_attack_damage(
                    amount.saturating_add(vigor),
                    str_stacks,
                    weak_factor(weak, false),
                    vuln_factor(false, false),
                );
                if double {
                    d = d.saturating_mul(2);
                }
                PyEffect::DamagePhysical(PyEffectDamagePhysical { amount: d, target })
            }
            PyEffect::BlockGain(PyEffectBlockGain { amount, target }) => {
                PyEffect::BlockGain(PyEffectBlockGain {
                    amount: scale_block_gain(amount, dex, frail),
                    target,
                })
            }
            other => other,
        })
        .collect()
}

fn snapshot_card(state: &GameState, id_card: usize) -> PyCard {
    let card = &state.entities[id_card];
    let entangled = has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Entangled,
    );
    // Combat-only; outside combat defaults are permissive (cards not played)
    let (restriction_ok, this_turn_discards, this_combat_damage, energy_current) =
        if let Mode::Combat {
            id_pile_draw,
            energy,
            this_turn_discards,
            this_combat_damage_instances_taken,
            ..
        } = &state.mode
        {
            (
                is_play_restriction_satisfied(
                    card.card_play_restriction,
                    card.card_kind,
                    &id_pile_draw,
                    &state.id_relics,
                ),
                *this_turn_discards,
                *this_combat_damage_instances_taken,
                energy.energy_current,
            )
        } else {
            (true, 0, 0, 0)
        };
    let entangled_blocks = entangled && card.card_kind == CardKind::Attack;
    let base = card.card_name.as_str();
    let display_name = if card.card_upgraded {
        format!("{base}+")
    } else {
        base.to_string()
    };
    let mut py_card = PyCard {
        name: card.card_name.into(),
        display_name,
        cost: get_card_effective_cost(card, this_turn_discards, this_combat_damage, energy_current),
        cost_base: card.card_cost,
        cost_zero_once: card.card_free_to_play_once,
        cost_override: card.card_cost_override,
        cost_kind: card.card_cost_kind.into(),
        kind: card.card_kind.into(),
        color: card.card_color.into(),
        rarity: card.card_rarity.into(),
        play_restriction: card.card_play_restriction.into(),
        upgraded: card.card_upgraded,
        exhaust: card.card_exhaust,
        ethereal: card.card_ethereal,
        innate: card.card_innate,
        requires_target: card.requires_target,
        retain: card.card_retain,
        playable: restriction_ok && !entangled_blocks,
        effects: snapshot_adjusted_effects(card, &state.entities[state.id_character].modifiers),
        identity_hash: 0,
    };
    py_card.identity_hash = card_identity_hash(&py_card);
    py_card
}

// Fingerprint over the snapshot fields clients key identity on. DefaultHasher::new()
// uses fixed keys, so the value is deterministic across processes.
fn card_identity_hash(card: &PyCard) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut h = DefaultHasher::new();
    card.name.hash(&mut h);
    card.kind.hash(&mut h);
    card.color.hash(&mut h);
    card.rarity.hash(&mut h);
    card.cost_kind.hash(&mut h);
    card.cost.hash(&mut h);
    card.cost_base.hash(&mut h);
    card.cost_zero_once.hash(&mut h);
    card.cost_override.hash(&mut h);
    card.upgraded.hash(&mut h);
    card.exhaust.hash(&mut h);
    card.innate.hash(&mut h);
    card.ethereal.hash(&mut h);
    card.retain.hash(&mut h);
    card.requires_target.hash(&mut h);
    card.playable.hash(&mut h);
    card.effects.hash(&mut h);
    h.finish()
}

// Position-independent hash of the room topology (kinds + edges) — a stable map identity for the
// RL encoder's static-grid cache. Excludes the live position so it's constant across a map's life.
fn map_identity_hash(state: &GameState) -> u64 {
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for (y, row) in state.id_rooms.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(id_room) = *cell {
                let room = &state.entities[id_room];
                (y, x, room.room_kind, room.room_edges).hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

fn snapshot_map(state: &GameState) -> PyMap {
    let rooms = state
        .id_rooms
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    cell.map(|id_room| {
                        let room = &state.entities[id_room];
                        PyRoom {
                            room_kind: room.room_kind.into(),
                            edges: edge_indices(room.room_edges).collect(),
                            chest_opened: room.room_chest_opened,
                        }
                    })
                })
                .collect()
        })
        .collect();

    let (y_current, x_current) = match state.location {
        Location::Start => (None, None),
        Location::Overworld { y, x } => (Some(y), Some(x)),
        Location::BossRoom => (Some(MAP_HEIGHT), Some(0)),
    };
    PyMap {
        rooms,
        y_current,
        x_current,
        boss: state.encounter_boss.into(),
        identity_hash: map_identity_hash(state),
    }
}

// pyo3's derived `hash` runs the discriminant through a hasher, so hash(enum) != hash(int)
// even though `eq_int` makes enum == int. That violates Python's eq/hash contract and makes
// these enums silently un-findable in int/IntEnum-keyed dicts. Hash by the raw discriminant
// so eq and hash agree.
macro_rules! impl_discriminant_hash {
    ($($ty:ty),+ $(,)?) => {
        $(
            #[pymethods]
            impl $ty {
                fn __hash__(&self) -> isize {
                    *self as isize
                }
            }
        )+
    };
}

impl_discriminant_hash!(
    PyCardKind,
    PyCardColor,
    PyCardRarity,
    PyPlayRestriction,
    PyDeltaSign,
    PyRoomKind,
    PyPotionName,
    PyPotionRarity,
    PyRelicName,
    PyCardName,
    PyMonsterName,
    PyMonsterEncounter,
    PyRelicTier,
    PyCandidatePoolMonstersFilter,
    PyCandidatePoolCardFilter,
    PyIntentKind,
);
