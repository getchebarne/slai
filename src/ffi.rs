// FFI boundary: all #[pyclass] types live here, named `Py<X>` to mirror the engine `X`
use pyo3::prelude::*;

use crate::action::Action;
use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::consts::MAP_HEIGHT;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::effect::input_count;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::events::event_option_gate_satisfied;
use crate::game::GameState;
use crate::game::Location;
use crate::map::edge_indices;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_has;
use crate::modifier::modifier_kind_from_u8;
use crate::modifier::modifier_stacks;
use crate::modifier::stacks_max_for;
use crate::monsters::hexaghost;
use crate::relics::iter_owned_relics;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::ChestKind;
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

// Enum mirrors

#[pyclass(eq, eq_int, hash, frozen, name = "CardKind")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "CardColor")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "CardRarity")]
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

#[pyclass(eq, hash, frozen, name = "CardCostKind")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "HealthDeltaSign")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyHealthDeltaSign {
    Gain,
    Loss,
}

impl From<HealthDeltaSign> for PyHealthDeltaSign {
    fn from(sign: HealthDeltaSign) -> Self {
        match sign {
            HealthDeltaSign::Gain => Self::Gain,
            HealthDeltaSign::Loss => Self::Loss,
        }
    }
}

#[pyclass(eq, hash, frozen, name = "HealthDeltaAmount")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "RoomKind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyRoomKind {
    CombatMonster,
    CombatElite,
    CombatBoss,
    RestSite,
    Treasure,
    EventRoom,
    Shop,
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
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "ChestKind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyChestKind {
    Small,
    Medium,
    Large,
}

impl From<ChestKind> for PyChestKind {
    fn from(kind: ChestKind) -> Self {
        match kind {
            ChestKind::Small => Self::Small,
            ChestKind::Medium => Self::Medium,
            ChestKind::Large => Self::Large,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "PotionName")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "PotionRarity")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "RelicName")]
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
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "CardName")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "MonsterName")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "EventName")]
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
    WeMeetAgain,
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
            EventName::WeMeetAgain => Self::WeMeetAgain,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "RelicTier")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "ModifierKind")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "CandidatePool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCandidatePool {
    Hand,
    Character,
    Monsters,
    Source,
    NextRowRooms,
    IdPick,
    Deck,
}

impl From<CandidatePool> for PyCandidatePool {
    fn from(pool: CandidatePool) -> Self {
        match pool {
            CandidatePool::Hand => Self::Hand,
            CandidatePool::Character => Self::Character,
            CandidatePool::Monsters { filter: _ } => Self::Monsters,
            CandidatePool::Source => Self::Source,
            CandidatePool::NextRowRooms => Self::NextRowRooms,
            CandidatePool::IdPick => Self::IdPick,
            CandidatePool::Deck { filter: _ } => Self::Deck,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "CandidatePoolMonstersFilter")]
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

// Phase / Selection / Target

#[pyclass(eq, eq_int, hash, frozen, name = "Screen")]
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

#[pyclass(eq, eq_int, hash, frozen, name = "CandidatePoolDeckFilter")]
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

#[pyclass(eq, hash, frozen, name = "SelectionKind")]
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

#[pyclass(eq, hash, frozen, get_all, name = "Target")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyTarget {
    pub candidate_pool: PyCandidatePool,
    pub selection_kind: PySelectionKind,
}

// Action

// `PyActionType` is the discriminant for the flat `PyAction` struct below
#[pyclass(eq, eq_int, hash, frozen, name = "ActionType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyActionType {
    CardPlay,
    EndTurn,
    HandSelect,
    RoomSelect,
    RestSiteRest,
    RestSiteCardUpgrade,
    RoomSkip,
    ChestOpen,
    PotionUse,
    PotionDiscard,
    CardDiscoverSelect,
    RewardTakeCard,
    RewardTakeRelic,
    RewardTakePotion,
    RewardTakeGold,
    RewardSkip,
    EventChoice,
    DeckSelect,
}

impl PyActionType {
    fn from_discriminant(discriminant: u8) -> Result<Self, String> {
        match discriminant {
            0 => Ok(Self::CardPlay),
            1 => Ok(Self::EndTurn),
            2 => Ok(Self::HandSelect),
            3 => Ok(Self::RoomSelect),
            4 => Ok(Self::RestSiteRest),
            5 => Ok(Self::RestSiteCardUpgrade),
            6 => Ok(Self::RoomSkip),
            7 => Ok(Self::ChestOpen),
            8 => Ok(Self::PotionUse),
            9 => Ok(Self::PotionDiscard),
            10 => Ok(Self::CardDiscoverSelect),
            11 => Ok(Self::RewardTakeCard),
            12 => Ok(Self::RewardTakeRelic),
            13 => Ok(Self::RewardTakePotion),
            14 => Ok(Self::RewardTakeGold),
            15 => Ok(Self::RewardSkip),
            16 => Ok(Self::EventChoice),
            17 => Ok(Self::DeckSelect),
            _ => Err(format!("PyActionType: invalid discriminant {discriminant}")),
        }
    }
}

#[pyclass(eq, hash, frozen, name = "Action")]
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
    fn new(action_type: u8, idxs: Vec<usize>, kind: Option<u8>) -> PyResult<Self> {
        let action_type = PyActionType::from_discriminant(action_type)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(Self {
            action_type,
            idxs,
            kind,
        })
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
                idx_hand: idxs[0],
                idx_monster: None,
            }),
            2 => Ok(Action::CardPlay {
                idx_hand: idxs[0],
                idx_monster: Some(idxs[1]),
            }),
            n => Err(format!(
                "CardPlay expects [idx_hand] or [idx_hand, idx_monster], got {n} idxs"
            )),
        },
        PyActionType::EndTurn => match idxs.len() {
            0 => Ok(Action::EndTurn),
            n => Err(format!("EndTurn expects [], got {n} idxs")),
        },
        PyActionType::HandSelect => Ok(Action::HandSelect { idxs: idxs.clone() }),
        PyActionType::RoomSelect => match idxs.len() {
            1 => Ok(Action::RoomSelect {
                idx_column: idxs[0],
            }),
            n => Err(format!("RoomSelect expects [idx_column], got {n} idxs")),
        },
        PyActionType::RestSiteRest => match idxs.len() {
            0 => Ok(Action::RestSiteRest),
            n => Err(format!("RestSiteRest expects [], got {n} idxs")),
        },
        PyActionType::RestSiteCardUpgrade => match idxs.len() {
            1 => Ok(Action::RestSiteCardUpgrade { idx_deck: idxs[0] }),
            n => Err(format!(
                "RestSiteCardUpgrade expects [idx_deck], got {n} idxs"
            )),
        },
        PyActionType::RoomSkip => match idxs.len() {
            0 => Ok(Action::RoomSkip),
            n => Err(format!("RoomSkip expects [], got {n} idxs")),
        },
        PyActionType::ChestOpen => match idxs.len() {
            0 => Ok(Action::ChestOpen),
            n => Err(format!("ChestOpen expects [], got {n} idxs")),
        },
        PyActionType::PotionUse => match idxs.len() {
            1 => Ok(Action::PotionUse {
                idx_slot: idxs[0],
                idx_monster: None,
            }),
            2 => Ok(Action::PotionUse {
                idx_slot: idxs[0],
                idx_monster: Some(idxs[1]),
            }),
            n => Err(format!(
                "PotionUse expects [idx_slot] or [idx_slot, idx_monster], got {n} idxs"
            )),
        },
        PyActionType::PotionDiscard => match idxs.len() {
            1 => Ok(Action::PotionDiscard { idx_slot: idxs[0] }),
            n => Err(format!("PotionDiscard expects [idx_slot], got {n} idxs")),
        },
        PyActionType::CardDiscoverSelect => match idxs.len() {
            1 => Ok(Action::CardDiscoverSelect {
                idx_option: idxs[0],
            }),
            n => Err(format!(
                "CardDiscoverSelect expects [idx_option], got {n} idxs"
            )),
        },
        PyActionType::RewardTakeCard => match idxs.len() {
            1 => Ok(Action::RewardTakeCard {
                idx_reward: idxs[0],
            }),
            n => Err(format!("RewardTakeCard expects [idx_reward], got {n} idxs")),
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
        PyActionType::RewardSkip => match idxs.len() {
            0 => Ok(Action::RewardSkip),
            n => Err(format!("RewardSkip expects [], got {n} idxs")),
        },
        PyActionType::EventChoice => match idxs.len() {
            1 => Ok(Action::EventChoice {
                idx_option: idxs[0],
            }),
            n => Err(format!("EventChoice expects [idx_option], got {n} idxs")),
        },
        PyActionType::DeckSelect => match idxs.len() {
            1 => Ok(Action::DeckSelect {
                idx_option: idxs[0],
            }),
            n => Err(format!("DeckSelect expects [idx_option], got {n} idxs")),
        },
    }
}

pub fn from_internal_action(action: Action) -> PyAction {
    let (action_type, idxs) = match action {
        Action::CardPlay {
            idx_hand,
            idx_monster: None,
        } => (PyActionType::CardPlay, vec![idx_hand]),
        Action::CardPlay {
            idx_hand,
            idx_monster: Some(m),
        } => (PyActionType::CardPlay, vec![idx_hand, m]),
        Action::EndTurn => (PyActionType::EndTurn, vec![]),
        Action::HandSelect { idxs } => (PyActionType::HandSelect, idxs),
        Action::RoomSelect { idx_column } => (PyActionType::RoomSelect, vec![idx_column]),
        Action::RestSiteRest => (PyActionType::RestSiteRest, vec![]),
        Action::RestSiteCardUpgrade { idx_deck } => {
            (PyActionType::RestSiteCardUpgrade, vec![idx_deck])
        }
        Action::RoomSkip => (PyActionType::RoomSkip, vec![]),
        Action::ChestOpen => (PyActionType::ChestOpen, vec![]),
        Action::PotionUse {
            idx_slot,
            idx_monster: None,
        } => (PyActionType::PotionUse, vec![idx_slot]),
        Action::PotionUse {
            idx_slot,
            idx_monster: Some(m),
        } => (PyActionType::PotionUse, vec![idx_slot, m]),
        Action::PotionDiscard { idx_slot } => (PyActionType::PotionDiscard, vec![idx_slot]),
        Action::CardDiscoverSelect { idx_option } => {
            (PyActionType::CardDiscoverSelect, vec![idx_option])
        }
        Action::RewardTakeCard { idx_reward } => (PyActionType::RewardTakeCard, vec![idx_reward]),
        Action::RewardTakeRelic => (PyActionType::RewardTakeRelic, vec![]),
        Action::RewardTakePotion => (PyActionType::RewardTakePotion, vec![]),
        Action::RewardTakeGold => (PyActionType::RewardTakeGold, vec![]),
        Action::RewardSkip => (PyActionType::RewardSkip, vec![]),
        Action::EventChoice { idx_option } => (PyActionType::EventChoice, vec![idx_option]),
        Action::DeckSelect { idx_option } => (PyActionType::DeckSelect, vec![idx_option]),
    };
    PyAction {
        action_type,
        idxs,
        kind: None,
    }
}

// Mirrors only EffectKind variants reachable from static card/monster defs; snapshot_effect panics on runtime-only variants
#[pyclass(eq, hash, frozen, name = "Effect")]
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
    FinisherDamage {
        damage: u16,
        target: Option<PyTarget>,
    },
    FlechettesDamage {
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
    DrawUpTo {
        amount: u8,
        target: Option<PyTarget>,
    },
    CardDiscard {
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
        sign: PyHealthDeltaSign,
        amount: PyHealthDeltaAmount,
        target: Option<PyTarget>,
    },
    HealthDelta {
        sign: PyHealthDeltaSign,
        amount: PyHealthDeltaAmount,
        target: Option<PyTarget>,
    },
    PotionAddRandom {
        limited: bool,
        target: Option<PyTarget>,
    },
    CardDiscoverSelect {
        kind: PyCardKind,
        count: u8,
        target: Option<PyTarget>,
    },
    GoldLoss {
        amount: u16,
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
    RollD100Branch {
        chance: u8,
        on_lt: Vec<PyEffect>,
        on_ge: Vec<PyEffect>,
        target: Option<PyTarget>,
    },
    EventEnd {
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
    GoldGain {
        amount: u16,
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
        EffectKind::FinisherDamage { damage } => PyEffect::FinisherDamage { damage, target },
        EffectKind::FlechettesDamage { damage } => PyEffect::FlechettesDamage { damage, target },
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
        EffectKind::DrawUpTo { amount } => PyEffect::DrawUpTo { amount, target },
        EffectKind::CardDiscard { source: _ } => PyEffect::CardDiscard { target },
        EffectKind::DamageMindBlast => PyEffect::DamageMindBlast { target },
        EffectKind::ShuffleDiscardPileIntoDrawPile => {
            PyEffect::ShuffleDiscardPileIntoDrawPile { target }
        }
        EffectKind::CalculatedGamble => PyEffect::CalculatedGamble { target },
        EffectKind::GoldLoss { amount } => PyEffect::GoldLoss { amount, target },
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
        EffectKind::RollD100Branch {
            chance,
            on_lt,
            on_ge,
        } => PyEffect::RollD100Branch {
            chance,
            on_lt: on_lt.iter().map(snapshot_effect).collect(),
            on_ge: on_ge.iter().map(snapshot_effect).collect(),
            target,
        },
        EffectKind::EventEnd => PyEffect::EventEnd { target },
        EffectKind::CardAddToDeck {
            card_name,
            upgraded,
        } => PyEffect::CardAddToDeck {
            card_name: card_name.as_str().to_string(),
            upgraded,
            target,
        },
        EffectKind::GoldGain { amount } => PyEffect::GoldGain { amount, target },
        EffectKind::PotionAddRandom { limited } => PyEffect::PotionAddRandom { limited, target },
        EffectKind::CardDiscoverSelect { kind, count } => PyEffect::CardDiscoverSelect {
            kind: kind.into(),
            count,
            target,
        },
        EffectKind::CardUpgrade => PyEffect::CardUpgrade { target },
        other => unreachable!(
            "snapshot_effect: unexpected EffectKind on static card effect: {:?}",
            other
        ),
    }
}

// Exposed structs
#[pyclass(frozen, get_all, name = "Card")]
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

    // Other boolean fields
    pub upgraded: bool,
    pub exhaust: bool,
    pub ethereal: bool,
    pub innate: bool,
    pub requires_target: bool,
    pub retain: bool,
    // `playable` does NOT factor in energy cost; clients must also check `cost <= energy.current`
    pub playable: bool,

    // Effects
    pub effects: Vec<PyEffect>,
}

#[pyclass(frozen, get_all, name = "Modifier")]
#[derive(Debug, Clone)]
pub struct PyModifier {
    pub kind: PyModifierKind,
    pub stacks: i16,
    pub stacks_max: i16,
}

#[pyclass(frozen, get_all, name = "Relic")]
#[derive(Debug, Clone)]
pub struct PyRelic {
    pub name: PyRelicName,
    pub tier: PyRelicTier,
    pub counter: i16,
    pub used_up: bool,
}

#[pyclass(frozen, get_all, name = "Potion")]
#[derive(Debug, Clone)]
pub struct PyPotion {
    pub name: PyPotionName,
    pub rarity: PyPotionRarity,
    pub requires_target: bool,
    pub combat_only: bool,
    pub effects: Vec<PyEffect>,
}

#[pyclass(frozen, get_all, name = "EventOption")]
#[derive(Debug, Clone)]
pub struct PyEventOption {
    pub label: String,
    pub gated_out: bool,
    pub effects: Vec<PyEffect>,
}

#[pyclass(frozen, get_all, name = "Event")]
#[derive(Debug, Clone)]
pub struct PyEvent {
    pub name: PyEventName,
    pub display_name: String,
    pub options: Vec<PyEventOption>,
    pub state: u8,
}

#[pyclass(frozen, get_all, name = "Character")]
#[derive(Debug, Clone)]
pub struct PyCharacter {
    pub name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub gold: u16,
    pub potion_slots: Vec<Option<PyPotion>>,
    pub potion_slots_max: u8,
}

#[pyclass(eq, eq_int, hash, frozen, name = "IntentKind")]
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

#[pyclass(frozen, get_all, name = "Intent")]
#[derive(Debug, Clone)]
pub struct PyIntent {
    pub kind: PyIntentKind,
    pub damage: Option<u16>,
    pub instances: Option<u8>,
}

#[pyclass(frozen, get_all, name = "Monster")]
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

#[pyclass(frozen, get_all, name = "Energy")]
#[derive(Debug, Clone)]
pub struct PyEnergy {
    pub current: u8,
    pub max: u8,
}

#[pyclass(frozen, get_all, name = "Room")]
#[derive(Debug, Clone)]
pub struct PyRoom {
    pub room_kind: PyRoomKind,
    pub edges: Vec<usize>,
    pub chest_kind: Option<PyChestKind>,
    pub chest_opened: bool,
}

#[pyclass(frozen, get_all, name = "Map")]
#[derive(Debug, Clone)]
pub struct PyMap {
    pub rooms: Vec<Vec<Option<PyRoom>>>,
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
    pub boss_name: String,
}

#[pyclass(frozen, get_all, name = "GameState")]
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
    pub energy: PyEnergy,
    pub map: PyMap,
    pub reward: Option<PyReward>,
    pub event: Option<PyEvent>,
    pub pending_input: Option<PyPendingInput>,
}

#[pyclass(frozen, get_all, name = "Reward")]
#[derive(Debug, Clone)]
pub struct PyReward {
    pub cards: Vec<PyCard>,
    pub relic: Option<PyRelic>,
    pub potion: Option<PyPotion>,
    pub gold: Option<u16>,
}

// Halt overlay snapshot; `None` outside halts. Variants 1:1 with halting EffectKind
#[pyclass(frozen, name = "PendingInput")]
#[derive(Debug, Clone)]
pub enum PyPendingInput {
    Discard {
        num: u8,
    },
    Retain {
        num: u8,
    },
    Setup {},
    Nightmare {},
    Discover {
        cards: Vec<PyCard>,
    },
    DeckSelect {
        filter: PyCandidatePoolDeckFilter,
        cards: Vec<PyCard>,
    },
    RoomSelect {},
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
            Self::WeMeetAgain => "We Meet Again",
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
                    current: state.energy.current,
                    max: state.energy.max,
                },
            )
        } else {
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                PyEnergy { current: 0, max: 0 },
            )
        };
    let pending_input = snapshot_pending_input(state);
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
        energy,
        map: snapshot_map(state),
        screen: state.screen.into(),
        game_over: state.game_over,
        reward,
        event,
        pending_input,
    }
}

fn snapshot_pending_input(state: &GameState) -> Option<PyPendingInput> {
    let pending = state.pending_effect.as_ref()?;
    let num = input_count(pending).map(|c| c as u8);
    Some(match pending.kind {
        EffectKind::CardDiscard { .. } => PyPendingInput::Discard {
            num: num.unwrap_or(1),
        },
        EffectKind::CardRetain => PyPendingInput::Retain {
            num: num.unwrap_or(1),
        },
        EffectKind::CardSetupPick => PyPendingInput::Setup {},
        EffectKind::CardNightmarePick => PyPendingInput::Nightmare {},
        EffectKind::CardDiscoverPick => {
            let cards = if matches!(state.screen, Screen::Combat) {
                state
                    .id_pick
                    .iter()
                    .map(|&id| snapshot_card(state, id))
                    .collect()
            } else {
                Vec::new()
            };
            PyPendingInput::Discover { cards }
        }
        EffectKind::CardPurge
        | EffectKind::CardUpgrade
        | EffectKind::CardDuplicate
        | EffectKind::CardTransform => {
            let Target::Resolve {
                candidate_pool: CandidatePool::Deck { filter },
                ..
            } = pending.target
            else {
                unreachable!("deck-pick halt without Deck pool: {:?}", pending.target);
            };
            // buf_candidates was populated by resolve_or_halt at halt time
            let cards: Vec<PyCard> = state
                .buf_candidates
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect();
            PyPendingInput::DeckSelect {
                filter: filter.into(),
                cards,
            }
        }
        EffectKind::RoomSelect => PyPendingInput::RoomSelect {},
        _ => unreachable!("pending_effect with non-halting kind: {:?}", pending.kind),
    })
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
    let potion_slots = character.potion_slots[..character.potion_slots_max as usize]
        .iter()
        .map(|s| s.map(|id| snapshot_potion(&state.entities[id])))
        .collect();
    PyCharacter {
        name: character.character_name.to_string(),
        health: character.vitals.health,
        health_max: character.vitals.health_max,
        block: character.vitals.block,
        modifiers: snapshot_modifiers(&character.modifiers),
        gold: character.character_gold,
        potion_slots,
        potion_slots_max: character.potion_slots_max,
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

            let intent = if let Some(move_idx) = m.move_current {
                let mv = &m.moves[move_idx];
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
                state.energy.current,
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
    PyCard {
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
        upgraded: card.card_upgraded,
        exhaust: card.card_exhaust,
        ethereal: card.card_ethereal,
        innate: card.card_innate,
        requires_target: card.requires_target,
        retain: card.card_retain,
        playable: restriction_ok && !entangled_blocks,
        effects: card.card_effects[..card.card_effects_len as usize]
            .iter()
            .map(snapshot_effect)
            .collect(),
    }
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
                            edges: edge_indices(room.edges).collect(),
                            chest_kind: room.room_chest_kind.map(Into::into),
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
        boss_name: state.encounter_boss.as_str().to_string(),
    }
}
