use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use pyo3_stub_gen::derive::gen_stub_pyclass_complex_enum;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;
use pyo3_stub_gen::derive::gen_stub_pymethods;

use crate::action::Action;
use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::consts::MAP_HEIGHT;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::GoldDeltaKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::PlayRestriction;
use crate::entity::card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::events::event_option_gate_satisfied;
use crate::game::GameState;
use crate::game::Location;
use crate::map::edge_indices;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_has;
use crate::modifier::modifier_is_buff;
use crate::modifier::modifier_kind_from_u8;
use crate::modifier::modifier_stacks;
use crate::modifier::stacks_max_for;
use crate::monsters::hexaghost;
use crate::relics::iter_owned_relics;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::MonsterEncounter;
use crate::types::MonsterName;
use crate::types::PotionName;
use crate::types::PotionRarity;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;
use crate::types::Screen;
use crate::utils::scale_attack_damage;
use crate::utils::scale_block_gain;

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_complex_enum]
#[pyclass(eq, hash, frozen, name = "CardCostKind", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyCardCostKind {
    Fixed {},
    MinusDiscardsThisTurn {},
    GrowsOnDamageInstanceTaken {},
    XCost { offset: i8 },
}

impl From<CardCostKind> for PyCardCostKind {
    fn from(kind: CardCostKind) -> Self {
        match kind {
            CardCostKind::Fixed => Self::Fixed {},
            CardCostKind::MinusDiscardsThisTurn => Self::MinusDiscardsThisTurn {},
            CardCostKind::GrowsOnDamageInstanceTaken => Self::GrowsOnDamageInstanceTaken {},
            CardCostKind::XCost { offset } => Self::XCost { offset },
        }
    }
}

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_complex_enum]
#[pyclass(eq, hash, frozen, name = "HealthDeltaAmount", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyHealthDeltaAmount {
    Absolute { amount: u16 },
    Relative { numerator: u8, denominator: u8 },
}

impl From<HealthDeltaAmount> for PyHealthDeltaAmount {
    fn from(amount: HealthDeltaAmount) -> Self {
        match amount {
            HealthDeltaAmount::Absolute(amount) => Self::Absolute { amount },
            HealthDeltaAmount::Relative {
                numerator,
                denominator,
            } => Self::Relative {
                numerator,
                denominator,
            },
        }
    }
}

#[gen_stub_pyclass_complex_enum]
#[pyclass(eq, hash, frozen, name = "GoldDeltaKind", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyGoldDeltaKind {
    Fixed { amount: u16 },
    Range { min: u16, max: u16 },
}

impl From<GoldDeltaKind> for PyGoldDeltaKind {
    fn from(kind: GoldDeltaKind) -> Self {
        match kind {
            GoldDeltaKind::Fixed(amount) => Self::Fixed { amount },
            GoldDeltaKind::Range { min, max } => Self::Range { min, max },
        }
    }
}

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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
        }
    }
}

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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
        }
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int, frozen, name = "EventName", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyEventName {
    BigFish,
    TheCleric,
    Duplicator,
    GoldenShrine,
    GoldenIdol,
    WingStatue,
    WorldOfGoop,
    LivingWall,
    Purifier,
    ScrapOoze,
    ShiningLight,
    TheSsssserpent,
    Transmogrifier,
    UpgradeShrine,
}

impl From<EventName> for PyEventName {
    fn from(name: EventName) -> Self {
        match name {
            EventName::BigFish => Self::BigFish,
            EventName::TheCleric => Self::TheCleric,
            EventName::Duplicator => Self::Duplicator,
            EventName::GoldenShrine => Self::GoldenShrine,
            EventName::GoldenIdol => Self::GoldenIdol,
            EventName::WingStatue => Self::WingStatue,
            EventName::WorldOfGoop => Self::WorldOfGoop,
            EventName::LivingWall => Self::LivingWall,
            EventName::Purifier => Self::Purifier,
            EventName::ScrapOoze => Self::ScrapOoze,
            EventName::ShiningLight => Self::ShiningLight,
            EventName::TheSsssserpent => Self::TheSsssserpent,
            EventName::Transmogrifier => Self::Transmogrifier,
            EventName::UpgradeShrine => Self::UpgradeShrine,
        }
    }
}

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
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
}

#[gen_stub_pymethods]
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
        }
    }
}

#[gen_stub_pyclass_complex_enum]
#[pyclass(eq, hash, frozen, name = "CandidatePool", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyCandidatePool {
    Hand {},
    Character {},
    Monsters {
        filter: PyCandidatePoolMonstersFilter,
    },
    Source {},
    Discover {},
    Deck {
        filter: PyCandidatePoolDeckFilter,
    },
}

impl From<CandidatePool> for PyCandidatePool {
    fn from(pool: CandidatePool) -> Self {
        match pool {
            CandidatePool::Hand => Self::Hand {},
            CandidatePool::Character => Self::Character {},
            CandidatePool::Monsters { filter } => Self::Monsters {
                filter: filter.into(),
            },
            CandidatePool::Source => Self::Source {},
            CandidatePool::Discover => Self::Discover {},
            CandidatePool::Deck { filter } => Self::Deck {
                filter: filter.into(),
            },
        }
    }
}

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass_enum]
#[pyclass(eq, eq_int, frozen, name = "Screen", module = "slai.slai")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyScreen {
    Combat,
    Reward,
    Event,
    Shop,
    Map,
    RestSite,
    Chest,
}

impl From<Screen> for PyScreen {
    fn from(a: Screen) -> Self {
        match a {
            Screen::Combat => Self::Combat,
            Screen::Reward => Self::Reward,
            Screen::Event => Self::Event,
            Screen::Shop => Self::Shop,
            Screen::Map => Self::Map,
            Screen::RestSite => Self::RestSite,
            Screen::Chest => Self::Chest,
        }
    }
}

#[gen_stub_pyclass_enum]
#[pyclass(
    eq,
    eq_int,
    frozen,
    name = "CandidatePoolDeckFilter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCandidatePoolDeckFilter {
    Purgeable,
    Upgradeable,
    Any,
    Transformable,
}

impl From<CandidatePoolDeckFilter> for PyCandidatePoolDeckFilter {
    fn from(f: CandidatePoolDeckFilter) -> Self {
        match f {
            CandidatePoolDeckFilter::Purgeable => Self::Purgeable,
            CandidatePoolDeckFilter::Upgradeable => Self::Upgradeable,
            CandidatePoolDeckFilter::Any => Self::Any,
            CandidatePoolDeckFilter::Transformable => Self::Transformable,
        }
    }
}

#[gen_stub_pyclass_complex_enum]
#[pyclass(eq, hash, frozen, name = "SelectionKind", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PySelectionKind {
    All {},
    Single {},
    Random { count: u8 },
    Input { count: u16 },
}

impl From<SelectionKind> for PySelectionKind {
    fn from(selection_kind: SelectionKind) -> Self {
        match selection_kind {
            SelectionKind::All => Self::All {},
            SelectionKind::Single => Self::Single {},
            SelectionKind::Random { count } => Self::Random { count },
            SelectionKind::Input { count } => Self::Input { count },
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(eq, hash, frozen, get_all, name = "Target", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyTarget {
    pub candidate_pool: PyCandidatePool,
    pub selection_kind: PySelectionKind,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyTarget {
    #[new]
    fn new(candidate_pool: PyCandidatePool, selection_kind: PySelectionKind) -> Self {
        Self {
            candidate_pool,
            selection_kind,
        }
    }
}

// `PyActionType` is the discriminant for the flat `PyAction` struct below
#[gen_stub_pyclass_enum]
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

#[gen_stub_pymethods]
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

#[gen_stub_pyclass]
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

#[gen_stub_pymethods]
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

// Multi-index picks are order-insensitive; normalize to the sorted-distinct form the enumerator emits
fn canonical_idxs(idxs: &[usize]) -> Vec<usize> {
    let mut out = idxs.to_vec();
    out.sort_unstable();
    out.dedup();
    out
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
        PyActionType::CardDiscard => Ok(Action::CardDiscard {
            idxs: canonical_idxs(idxs),
        }),
        PyActionType::CardRetain => Ok(Action::CardRetain {
            idxs: canonical_idxs(idxs),
        }),
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
            0 => Ok(Action::RewardTakePotion),
            n => Err(format!("RewardTakePotion expects [], got {n} idxs")),
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
        Action::CardDiscard { idxs } => (PyActionType::CardDiscard, idxs),
        Action::CardRetain { idxs } => (PyActionType::CardRetain, idxs),
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
        Action::RewardTakePotion => (PyActionType::RewardTakePotion, vec![]),
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
#[gen_stub_pyclass_complex_enum]
#[pyclass(eq, hash, frozen, name = "Effect", module = "slai.slai")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyEffect {
    DamagePhysical {
        amount: u16,
        target: Option<PyTarget>,
    },
    DamagePhysicalIfPoisoned {
        amount: u16,
        target: Option<PyTarget>,
    },
    HeelHookProc {
        target: Option<PyTarget>,
    },
    EscapePlanCheck {
        block: u16,
        target: Option<PyTarget>,
    },
    GlassKnifeDecay {
        delta: i16,
        target: Option<PyTarget>,
    },
    CardSetupPick {
        target: Option<PyTarget>,
    },
    CardNightmarePick {
        target: Option<PyTarget>,
    },
    DistractionAdd {
        target: Option<PyTarget>,
    },
    SetCostOverride {
        amount: u8,
        target: Option<PyTarget>,
    },
    DamageFinisher {
        damage: u16,
        target: Option<PyTarget>,
    },
    DamageFlechettes {
        damage: u16,
        target: Option<PyTarget>,
    },
    UnloadDiscard {
        target: Option<PyTarget>,
    },
    StormOfSteelProc {
        upgraded: bool,
        target: Option<PyTarget>,
    },
    SneakyStrikeProc {
        energy: u8,
        target: Option<PyTarget>,
    },
    BlockGain {
        amount: u16,
        target: Option<PyTarget>,
    },
    ModifierGain {
        kind: PyModifierKind,
        stacks: i16,
        target: Option<PyTarget>,
    },
    ModifierMultiply {
        kind: PyModifierKind,
        factor: u8,
        target: Option<PyTarget>,
    },
    ModifierRemove {
        kind: PyModifierKind,
        target: Option<PyTarget>,
    },
    EnergyGain {
        amount: u16,
        target: Option<PyTarget>,
    },
    CardAddToHand {
        card_name: String,
        count: u16,
        upgraded: bool,
        target: Option<PyTarget>,
    },
    CardDraw {
        count: u16,
        target: Option<PyTarget>,
    },
    CardDrawUpTo {
        amount: u8,
        target: Option<PyTarget>,
    },
    CardDiscard {
        target: Option<PyTarget>,
    },
    CardRetain {
        target: Option<PyTarget>,
    },
    DamageMindBlast {
        target: Option<PyTarget>,
    },
    ShuffleDiscardPileIntoDrawPile {
        target: Option<PyTarget>,
    },
    CalculatedGamble {
        target: Option<PyTarget>,
    },
    MaxHealthDelta {
        sign: PyDeltaSign,
        amount: PyHealthDeltaAmount,
        target: Option<PyTarget>,
    },
    HealthDelta {
        sign: PyDeltaSign,
        amount: PyHealthDeltaAmount,
        target: Option<PyTarget>,
    },
    PotionAddRandom {
        limited: bool,
        target: Option<PyTarget>,
    },
    CardDiscoverRoll {
        kind: PyCardKind,
        count: u8,
        target: Option<PyTarget>,
    },
    GoldDelta {
        sign: PyDeltaSign,
        kind: PyGoldDeltaKind,
        target: Option<PyTarget>,
    },
    RelicGrantRandom {
        target: Option<PyTarget>,
    },
    RelicGrantSpecific {
        name: PyRelicName,
        fallback_circlet: bool,
        target: Option<PyTarget>,
    },
    EventAdvanceState {
        delta: i8,
        target: Option<PyTarget>,
    },
    ScrapOozeReach {
        dmg: u16,
        chance: u8,
        advance_on_miss: bool,
        target: Option<PyTarget>,
    },
    EventConsume {
        target: Option<PyTarget>,
    },
    CardDiscoverPick {
        target: Option<PyTarget>,
    },
    CardAddToDeck {
        card_name: String,
        upgraded: bool,
        target: Option<PyTarget>,
    },
    CardPurge {
        target: Option<PyTarget>,
    },
    CardUpgrade {
        target: Option<PyTarget>,
    },
    CardDuplicate {
        target: Option<PyTarget>,
    },
    CardTransform {
        target: Option<PyTarget>,
    },
}

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
        EffectKind::DamagePhysical { amount } => PyEffect::DamagePhysical { amount, target },
        EffectKind::DamagePhysicalIfPoisoned { amount } => {
            PyEffect::DamagePhysicalIfPoisoned { amount, target }
        }
        EffectKind::HeelHookProc => PyEffect::HeelHookProc { target },
        EffectKind::EscapePlanCheck { block } => PyEffect::EscapePlanCheck { block, target },
        EffectKind::GlassKnifeDecay { delta } => PyEffect::GlassKnifeDecay { delta, target },
        EffectKind::CardSetupPick => PyEffect::CardSetupPick { target },
        EffectKind::CardNightmarePick => PyEffect::CardNightmarePick { target },
        EffectKind::DistractionAdd => PyEffect::DistractionAdd { target },
        EffectKind::SetCostOverride { amount } => PyEffect::SetCostOverride { amount, target },
        EffectKind::DamageFinisher { damage } => PyEffect::DamageFinisher { damage, target },
        EffectKind::DamageFlechettes { damage } => PyEffect::DamageFlechettes { damage, target },
        EffectKind::UnloadDiscard => PyEffect::UnloadDiscard { target },
        EffectKind::StormOfSteelProc { upgraded } => {
            PyEffect::StormOfSteelProc { upgraded, target }
        }
        EffectKind::SneakyStrikeProc { energy } => PyEffect::SneakyStrikeProc { energy, target },
        EffectKind::BlockGain { amount } => PyEffect::BlockGain { amount, target },
        EffectKind::ModifierGain { kind, stacks } => PyEffect::ModifierGain {
            kind: kind.into(),
            stacks,
            target,
        },
        EffectKind::ModifierMultiply { kind, factor } => PyEffect::ModifierMultiply {
            kind: kind.into(),
            factor,
            target,
        },
        EffectKind::ModifierRemove { kind } => PyEffect::ModifierRemove {
            kind: kind.into(),
            target,
        },
        EffectKind::EnergyGain { amount } => PyEffect::EnergyGain { amount, target },
        EffectKind::CardAddToHand {
            card_name,
            count,
            upgraded,
        } => PyEffect::CardAddToHand {
            card_name: card_name.as_str().to_string(),
            count,
            upgraded,
            target,
        },
        EffectKind::CardDraw { count } => PyEffect::CardDraw { count, target },
        EffectKind::CardDrawUpTo { amount } => PyEffect::CardDrawUpTo { amount, target },
        EffectKind::CardDiscard { source: _ } => PyEffect::CardDiscard { target },
        EffectKind::CardRetain => PyEffect::CardRetain { target },
        EffectKind::DamageMindBlast => PyEffect::DamageMindBlast { target },
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            PyEffect::ShuffleDiscardPileIntoDrawPile { target }
        }
        EffectKind::CalculatedGamble => PyEffect::CalculatedGamble { target },
        EffectKind::GoldDelta { sign, kind } => PyEffect::GoldDelta {
            sign: sign.into(),
            kind: kind.into(),
            target,
        },
        EffectKind::HealthDelta { sign, amount } => PyEffect::HealthDelta {
            sign: sign.into(),
            amount: amount.into(),
            target,
        },
        EffectKind::MaxHealthDelta { sign, amount } => PyEffect::MaxHealthDelta {
            sign: sign.into(),
            amount: amount.into(),
            target,
        },
        EffectKind::CardPurge => PyEffect::CardPurge { target },
        EffectKind::CardDuplicate => PyEffect::CardDuplicate { target },
        EffectKind::CardTransform => PyEffect::CardTransform { target },
        EffectKind::RelicGrantRandom => PyEffect::RelicGrantRandom { target },
        EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet,
        } => PyEffect::RelicGrantSpecific {
            name: name.into(),
            fallback_circlet,
            target,
        },
        EffectKind::EventAdvanceState { delta } => PyEffect::EventAdvanceState { delta, target },
        EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        } => PyEffect::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
            target,
        },
        EffectKind::EventConsume => PyEffect::EventConsume { target },
        EffectKind::CardAddToDeck {
            card_name,
            upgraded,
        } => PyEffect::CardAddToDeck {
            card_name: card_name.as_str().to_string(),
            upgraded,
            target,
        },
        EffectKind::PotionAddRandom { limited } => PyEffect::PotionAddRandom { limited, target },
        EffectKind::CardDiscoverRoll { kind, count } => PyEffect::CardDiscoverRoll {
            kind: kind.into(),
            count,
            target,
        },
        EffectKind::CardUpgrade => PyEffect::CardUpgrade { target },
        EffectKind::CardDiscoverPick => PyEffect::CardDiscoverPick { target },
        other => unreachable!(
            "snapshot_effect: unexpected EffectKind on static card effect: {:?}",
            other
        ),
    }
}

// Exposed structs
#[gen_stub_pyclass]
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

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Modifier", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModifier {
    pub kind: PyModifierKind,
    pub stacks: i16,
    pub stacks_max: i16,
}

#[gen_stub_pymethods]
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

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Relic", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyRelic {
    pub name: PyRelicName,
    pub tier: PyRelicTier,
    pub counter: i16,
    pub used_up: bool,
    pub effects_on_combat_start: Vec<PyEffect>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyRelic {
    #[new]
    fn new(
        name: PyRelicName,
        tier: PyRelicTier,
        counter: i16,
        used_up: bool,
        effects_on_combat_start: Vec<PyEffect>,
    ) -> Self {
        Self {
            name,
            tier,
            counter,
            used_up,
            effects_on_combat_start,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Potion", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyPotion {
    pub name: PyPotionName,
    pub rarity: PyPotionRarity,
    pub requires_target: bool,
    pub combat_only: bool,
    pub effects: Vec<PyEffect>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPotion {
    #[new]
    fn new(
        name: PyPotionName,
        rarity: PyPotionRarity,
        requires_target: bool,
        combat_only: bool,
        effects: Vec<PyEffect>,
    ) -> Self {
        Self {
            name,
            rarity,
            requires_target,
            combat_only,
            effects,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "EventOption", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEventOption {
    pub label: String,
    pub gated_out: bool,
    pub effects: Vec<PyEffect>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyEventOption {
    #[new]
    fn new(label: String, gated_out: bool, effects: Vec<PyEffect>) -> Self {
        Self {
            label,
            gated_out,
            effects,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Event", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEvent {
    pub name: PyEventName,
    pub display_name: String,
    pub options: Vec<PyEventOption>,
    pub state: u8,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyEvent {
    #[new]
    fn new(
        name: PyEventName,
        display_name: String,
        options: Vec<PyEventOption>,
        state: u8,
    ) -> Self {
        Self {
            name,
            display_name,
            options,
            state,
        }
    }
}

#[gen_stub_pyclass]
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

#[gen_stub_pymethods]
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

#[gen_stub_pyclass_enum]
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

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Intent", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyIntent {
    pub kind: PyIntentKind,
    pub damage: Option<u16>,
    pub instances: Option<u8>,
}

#[gen_stub_pymethods]
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

#[gen_stub_pyclass]
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

#[gen_stub_pymethods]
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

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Energy", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyEnergy {
    pub energy_current: u8,
    pub energy_max: u8,
}

#[gen_stub_pymethods]
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

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Room", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyRoom {
    pub room_kind: PyRoomKind,
    pub edges: Vec<usize>,
    pub chest_opened: bool,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyRoom {
    #[new]
    fn new(
        room_kind: PyRoomKind,
        edges: Vec<usize>,
        chest_opened: bool,
    ) -> Self {
        Self {
            room_kind,
            edges,
            chest_opened,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Map", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyMap {
    pub rooms: Vec<Vec<Option<PyRoom>>>,
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
    pub boss: PyMonsterEncounter,
    pub identity_hash: u64,
}

#[gen_stub_pymethods]
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

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "GameState", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyGameState {
    pub screen: PyScreen,
    pub game_over: bool,
    pub character: PyCharacter,
    pub monsters: Vec<PyMonster>,
    pub deck: Vec<PyCard>,
    pub hand: Vec<PyCard>,
    pub pile_draw: Vec<PyCard>,
    pub pile_discard: Vec<PyCard>,
    pub pile_exhaust: Vec<PyCard>,
    pub relics: Vec<PyRelic>,
    // Slot-indexed belt (length potion_slots_max); None at empty slots so positions stay valid
    pub potions: Vec<Option<PyPotion>>,
    pub potion_slots_max: u8,
    pub energy: PyEnergy,
    pub map: PyMap,
    pub reward: Option<PyReward>,
    pub event: Option<PyEvent>,
    pub pending: Option<PyEffect>,
    pub discover: Vec<PyCard>,
    pub shop: Option<PyShop>,
}

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Reward", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyReward {
    pub cards: Vec<PyCard>,
    pub relic: Option<PyRelic>,
    pub potion: Option<PyPotion>,
    pub gold: Option<u16>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyReward {
    #[new]
    fn new(
        cards: Vec<PyCard>,
        relic: Option<PyRelic>,
        potion: Option<PyPotion>,
        gold: Option<u16>,
    ) -> Self {
        Self {
            cards,
            relic,
            potion,
            gold,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass(frozen, get_all, name = "Shop", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyShop {
    pub cards: Vec<PyCard>,
    pub card_prices: Vec<u16>,
    pub relics: Vec<PyRelic>,
    pub relic_prices: Vec<u16>,
    pub potions: Vec<PyPotion>,
    pub potion_prices: Vec<u16>,
    pub purge_cost: u16,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyShop {
    #[new]
    fn new(
        cards: Vec<PyCard>,
        card_prices: Vec<u16>,
        relics: Vec<PyRelic>,
        relic_prices: Vec<u16>,
        potions: Vec<PyPotion>,
        potion_prices: Vec<u16>,
        purge_cost: u16,
    ) -> Self {
        Self {
            cards,
            card_prices,
            relics,
            relic_prices,
            potions,
            potion_prices,
            purge_cost,
        }
    }
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

impl EventName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BigFish => "Big Fish",
            Self::TheCleric => "The Cleric",
            Self::Duplicator => "Duplicator",
            Self::GoldenShrine => "Golden Shrine",
            Self::GoldenIdol => "Golden Idol",
            Self::WingStatue => "Wing Statue",
            Self::WorldOfGoop => "World of Goop",
            Self::LivingWall => "Living Wall",
            Self::Purifier => "Purifier",
            Self::ScrapOoze => "Scrap Ooze",
            Self::ShiningLight => "Shining Light",
            Self::TheSsssserpent => "The Ssssserpent",
            Self::Transmogrifier => "Transmogrifier",
            Self::UpgradeShrine => "Upgrade Shrine",
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
    // Combat-only fields default to empty / 0 when not in Combat context
    let (hand, pile_draw, pile_discard, pile_exhaust, energy) =
        if matches!(state.screen, Screen::Combat) {
            (
                state
                    .id_hand
                    .iter()
                    .map(|&id| snapshot_card(state, id))
                    .collect(),
                state
                    .id_pile_draw
                    .iter()
                    .map(|&id| snapshot_card(state, id))
                    .collect(),
                state
                    .id_pile_discard
                    .iter()
                    .map(|&id| snapshot_card(state, id))
                    .collect(),
                state
                    .id_pile_exhaust
                    .iter()
                    .map(|&id| snapshot_card(state, id))
                    .collect(),
                PyEnergy {
                    energy_current: state.energy.energy_current,
                    energy_max: state.energy.energy_max,
                },
            )
        } else {
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                PyEnergy {
                    energy_current: 0,
                    energy_max: 0,
                },
            )
        };
    let pending = state.effect_pending.as_ref().map(snapshot_effect);
    let discover: Vec<PyCard> = state
        .id_discover
        .iter()
        .map(|&id| snapshot_card(state, id))
        .collect();
    let reward = match state.screen {
        Screen::Reward => Some(PyReward {
            cards: state
                .reward_id_cards
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            relic: state
                .reward_id_relic
                .map(|id| snapshot_relic(&state.entities[id])),
            potion: state
                .reward_id_potion
                .map(|id| snapshot_potion(&state.entities[id])),
            gold: state.reward_gold,
        }),
        _ => None,
    };
    let event = match state.screen {
        Screen::Event => Some(snapshot_event(
            state,
            state
                .id_event
                .expect("Event context requires state.id_event"),
        )),
        _ => None,
    };
    let shop = match state.screen {
        Screen::Shop => Some(PyShop {
            cards: state
                .shop_id_cards
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            card_prices: state.shop_card_prices.clone(),
            relics: state
                .shop_id_relics
                .iter()
                .map(|&id| snapshot_relic(&state.entities[id]))
                .collect(),
            relic_prices: state.shop_relic_prices.clone(),
            potions: state
                .shop_id_potions
                .iter()
                .map(|&id| snapshot_potion(&state.entities[id]))
                .collect(),
            potion_prices: state.shop_potion_prices.clone(),
            purge_cost: state.shop_purge_cost,
        }),
        _ => None,
    };
    PyGameState {
        character: snapshot_character(state),
        monsters: snapshot_monsters(state),
        deck: state
            .id_deck
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        hand,
        pile_draw,
        pile_discard,
        pile_exhaust,
        relics: iter_owned_relics(&state.id_relics)
            .map(|(_name, id)| snapshot_relic(&state.entities[id]))
            .collect(),
        potions: state.id_potions[..state.potion_slots_max as usize]
            .iter()
            .map(|s| s.map(|id| snapshot_potion(&state.entities[id])))
            .collect(),
        potion_slots_max: state.potion_slots_max,
        energy,
        map: snapshot_map(state),
        screen: state.screen.into(),
        game_over: state.game_over,
        reward,
        event,
        pending,
        discover,
        shop,
    }
}

fn snapshot_event(state: &GameState, id_event: usize) -> PyEvent {
    let event = &state.entities[id_event];
    let options: Vec<PyEventOption> = event
        .event_options
        .iter()
        .map(|opt| PyEventOption {
            label: opt.label.to_string(),
            gated_out: !event_option_gate_satisfied(opt.gate, state, id_event),
            effects: opt.effects.iter().map(snapshot_effect).collect(),
        })
        .collect();
    PyEvent {
        name: event.event_name.into(),
        display_name: event.event_name.as_str().to_string(),
        options,
        state: event.event_state,
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
    let character = &state.entities[state.id_character];
    let mods_char = &character.modifiers;
    state
        .id_monsters
        .iter()
        .flatten()
        .copied()
        .map(|id_monster| {
            let m = &state.entities[id_monster];

            let intent = if let Some(move_idx) = m.monster_move_current {
                let mv = &m.monster_moves[move_idx];
                let (mut base_damage, mut instances) = match mv.intent {
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

                // Hexaghost Divider's per-hit damage is dynamic (HP/12 + 1); override the static placeholder
                if m.monster_name == MonsterName::Hexaghost
                    && move_idx == hexaghost::IDX_MOVE_DIVIDER
                {
                    base_damage = Some(character.vitals.health / 12 + 1);
                    instances = Some(HEXAGHOST_DIVIDER_HITS);
                }

                let damage = base_damage.map(|d| {
                    let str_stacks = if modifier_has(&m.modifiers, ModifierKind::Strength) {
                        modifier_stacks(&m.modifiers, ModifierKind::Strength)
                    } else {
                        0
                    };
                    let mut scaled = scale_attack_damage(
                        d,
                        str_stacks,
                        modifier_has(&m.modifiers, ModifierKind::Weak),
                        state.id_relics[RelicName::PaperKrane as usize].is_some(),
                        modifier_has(mods_char, ModifierKind::Vulnerable),
                    );
                    if modifier_has(mods_char, ModifierKind::Intangible) && scaled > 1 {
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
    let mut out = Vec::new();
    let mut bits = mods.active;
    while bits != 0 {
        let idx = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        let kind = modifier_kind_from_u8(idx as u8);
        out.push(PyModifier {
            kind: kind.into(),
            stacks: mods.stacks[idx],
            stacks_max: stacks_max_for(kind),
        });
    }
    out
}

// Snapshot a card's effects with the current player modifiers folded into the DamagePhysical /
// BlockGain amounts (target-agnostic — Vulnerable/Intangible depend on the L3 target chosen later),
// via the same scaling utils as the live pipeline. Other effect kinds pass through unchanged.
fn snapshot_adjusted_effects(card: &Entity, char_mods: &Modifiers) -> Vec<PyEffect> {
    let vigor = if modifier_has(char_mods, ModifierKind::Vigor) {
        modifier_stacks(char_mods, ModifierKind::Vigor).max(0) as u16
    } else {
        0
    };
    let str_stacks = if modifier_has(char_mods, ModifierKind::Strength) {
        modifier_stacks(char_mods, ModifierKind::Strength)
    } else {
        0
    };
    let weak = modifier_has(char_mods, ModifierKind::Weak);
    let double = modifier_has(char_mods, ModifierKind::DoubleDamage);
    let dex = if modifier_has(char_mods, ModifierKind::Dexterity) {
        modifier_stacks(char_mods, ModifierKind::Dexterity)
    } else {
        0
    };
    let frail = modifier_has(char_mods, ModifierKind::Frail);

    card.card_effects[..card.card_effects_len as usize]
        .iter()
        .map(snapshot_effect)
        .map(|effect| match effect {
            PyEffect::DamagePhysical { amount, target } => {
                // Player attacker: Paper Krane never applies
                let mut d = scale_attack_damage(
                    amount.saturating_add(vigor),
                    str_stacks,
                    weak,
                    false,
                    false,
                );
                if double {
                    d = d.saturating_mul(2);
                }
                PyEffect::DamagePhysical { amount: d, target }
            }
            PyEffect::BlockGain { amount, target } => PyEffect::BlockGain {
                amount: scale_block_gain(amount, dex, frail),
                target,
            },
            other => other,
        })
        .collect()
}

fn snapshot_card(state: &GameState, id_card: usize) -> PyCard {
    let card = &state.entities[id_card];
    let entangled = modifier_has(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Entangled,
    );
    // Combat-only; outside combat defaults are permissive (cards not played)
    let (restriction_ok, this_turn_discards, this_combat_damage, energy_current) =
        if matches!(state.screen, Screen::Combat) {
            (
                is_play_restriction_satisfied(card.card_play_restriction, &state.id_pile_draw),
                state.this_turn_discards,
                state.this_combat_damage_instances_taken,
                state.energy.energy_current,
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
        cost: card_effective_cost(card, this_turn_discards, this_combat_damage, energy_current),
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
    use std::hash::{Hash, Hasher};
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
    PyEventName,
    PyRelicTier,
    PyCandidatePoolMonstersFilter,
    PyScreen,
    PyCandidatePoolDeckFilter,
    PyIntentKind,
);

#[cfg(test)]
mod card_combat_tests {
    use super::PyEffect;
    use super::snapshot_adjusted_effects;
    use crate::cards::get_card;
    use crate::modifier::ModifierKind;
    use crate::modifier::Modifiers;
    use crate::modifier::modifier_apply;
    use crate::modifier::modifiers_new;
    use crate::types::CardName;

    fn mods(pairs: &[(ModifierKind, i16)]) -> Modifiers {
        let mut m = modifiers_new();
        for &(kind, stacks) in pairs {
            modifier_apply(&mut m, kind, stacks);
        }
        m
    }

    fn dmg(effects: &[PyEffect]) -> u16 {
        effects
            .iter()
            .filter_map(|e| match e {
                PyEffect::DamagePhysical { amount, .. } => Some(*amount),
                _ => None,
            })
            .sum()
    }

    fn block(effects: &[PyEffect]) -> u16 {
        effects
            .iter()
            .filter_map(|e| match e {
                PyEffect::BlockGain { amount, .. } => Some(*amount),
                _ => None,
            })
            .sum()
    }

    // Effects carry the player-modifier adjustment. Strike = DamagePhysical 6, Defend = BlockGain 5.
    // Covers the modifiers random Act-1 play can't reach (Strength/Vigor/DoubleDamage) + floor cases.
    #[test]
    fn snapshot_adjusted_effects_applies_player_modifiers() {
        let strike = get_card(CardName::Strike, false);
        let defend = get_card(CardName::Defend, false);

        // No modifiers -> base
        assert_eq!(dmg(&snapshot_adjusted_effects(&strike, &modifiers_new())), 6);
        assert_eq!(block(&snapshot_adjusted_effects(&defend, &modifiers_new())), 5);

        // Damage: Strength/Vigor add, Weak *0.75 (floor), DoubleDamage *2
        let s = |m| dmg(&snapshot_adjusted_effects(&strike, &m));
        assert_eq!(s(mods(&[(ModifierKind::Strength, 3)])), 9);
        assert_eq!(s(mods(&[(ModifierKind::Vigor, 5)])), 11);
        assert_eq!(s(mods(&[(ModifierKind::Weak, 1)])), 4);
        assert_eq!(s(mods(&[(ModifierKind::Strength, 3), (ModifierKind::Weak, 1)])), 6); // floor((6+3)*.75)
        assert_eq!(s(mods(&[(ModifierKind::DoubleDamage, 1)])), 12);

        // Block: Dexterity adds, Frail *0.75 (floor)
        let b = |m| block(&snapshot_adjusted_effects(&defend, &m));
        assert_eq!(b(mods(&[(ModifierKind::Dexterity, 2)])), 7);
        assert_eq!(b(mods(&[(ModifierKind::Frail, 1)])), 3);
        assert_eq!(b(mods(&[(ModifierKind::Dexterity, 2), (ModifierKind::Frail, 1)])), 5); // floor((5+2)*.75)
    }
}
