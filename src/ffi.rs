// FFI boundary: all #[pyclass] types live here, named `Py<X>` to mirror the engine `X`
use pyo3::prelude::*;

use crate::action::Action;
use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAX_MONSTERS;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::card_effective_cost;
use crate::entity::is_play_restriction_satisfied;
use crate::game::GameState;
use crate::game::Location;
use crate::map::edge_indices;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_has;
use crate::modifier::modifier_stacks;
use crate::modifier::stacks_max_for;
use crate::monsters::hexaghost;
use crate::relics::iter_owned_relics;
use crate::types::CardColor;
use crate::types::CardKind;
use crate::types::CardName;
use crate::types::CardRarity;
use crate::types::ChestKind;
use crate::types::MonsterEncounter;
use crate::types::MonsterName;
use crate::types::Phase;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::types::RoomKind;
use crate::utils::fill_alive_monster_ids;
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
    fn from(k: CardKind) -> Self {
        match k {
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
    fn from(c: CardColor) -> Self {
        match c {
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
    fn from(r: CardRarity) -> Self {
        match r {
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
    fn from(k: CardCostKind) -> Self {
        match k {
            CardCostKind::Fixed => Self::Fixed {},
            CardCostKind::MinusDiscardsThisTurn => Self::MinusDiscardsThisTurn {},
            CardCostKind::GrowsOnDamageInstanceTaken => Self::GrowsOnDamageInstanceTaken {},
            CardCostKind::XCost { offset } => Self::XCost { offset },
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
    fn from(r: RoomKind) -> Self {
        match r {
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
    fn from(c: ChestKind) -> Self {
        match c {
            ChestKind::Small => Self::Small,
            ChestKind::Medium => Self::Medium,
            ChestKind::Large => Self::Large,
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
    fn from(n: RelicName) -> Self {
        match n {
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
    fn from(n: PyRelicName) -> Self {
        match n {
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
    AllOutAttack,
    Backflip,
    Backstab,
    Bane,
    BladeDance,
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
    Finisher,
    Flechettes,
    FlyingKnee,
    Footwork,
    GlassKnife,
    GrandFinale,
    HeelHook,
    InfiniteBlades,
    LegSweep,
    Malaise,
    MasterfulStab,
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
    fn from(n: CardName) -> Self {
        match n {
            CardName::AThousandCuts => Self::AThousandCuts,
            CardName::Accuracy => Self::Accuracy,
            CardName::Acrobatics => Self::Acrobatics,
            CardName::Adrenaline => Self::Adrenaline,
            CardName::AfterImage => Self::AfterImage,
            CardName::AllOutAttack => Self::AllOutAttack,
            CardName::Backflip => Self::Backflip,
            CardName::Backstab => Self::Backstab,
            CardName::Bane => Self::Bane,
            CardName::BladeDance => Self::BladeDance,
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
            CardName::Finisher => Self::Finisher,
            CardName::Flechettes => Self::Flechettes,
            CardName::FlyingKnee => Self::FlyingKnee,
            CardName::Footwork => Self::Footwork,
            CardName::GlassKnife => Self::GlassKnife,
            CardName::GrandFinale => Self::GrandFinale,
            CardName::HeelHook => Self::HeelHook,
            CardName::InfiniteBlades => Self::InfiniteBlades,
            CardName::LegSweep => Self::LegSweep,
            CardName::Malaise => Self::Malaise,
            CardName::MasterfulStab => Self::MasterfulStab,
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
    fn from(n: MonsterName) -> Self {
        match n {
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
    fn from(t: RelicTier) -> Self {
        match t {
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
    fn from(k: ModifierKind) -> Self {
        match k {
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
    CardTarget,
    Character,
    Monsters,
    OtherMonsters,
    Source,
    NextRowRooms,
    CardRewardPool,
}

impl From<CandidatePool> for PyCandidatePool {
    fn from(c: CandidatePool) -> Self {
        match c {
            CandidatePool::Hand => Self::Hand,
            CandidatePool::CardTarget => Self::CardTarget,
            CandidatePool::Character => Self::Character,
            CandidatePool::Monsters => Self::Monsters,
            CandidatePool::OtherMonsters => Self::OtherMonsters,
            CandidatePool::Source => Self::Source,
            CandidatePool::NextRowRooms => Self::NextRowRooms,
            CandidatePool::CardRewardPool => Self::CardRewardPool,
        }
    }
}

// Phase / Selection / Target

#[pyclass(eq, hash, frozen, name = "Phase")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyPhase {
    Map {},
    CombatDefault {},
    CombatAwaitDiscard { num: u8 },
    CombatAwaitNightmare {},
    CombatAwaitRetain { num: u8 },
    CombatAwaitSetup {},
    CombatReward {},
    RestSite {},
    GameOver {},
    Chest {},
    EventRoom {},
    Shop {},
}

impl From<Phase> for PyPhase {
    fn from(p: Phase) -> Self {
        match p {
            Phase::Map => Self::Map {},
            Phase::CombatDefault => Self::CombatDefault {},
            Phase::CombatAwaitDiscard { num } => Self::CombatAwaitDiscard { num },
            Phase::CombatAwaitNightmare => Self::CombatAwaitNightmare {},
            Phase::CombatAwaitRetain { num } => Self::CombatAwaitRetain { num },
            Phase::CombatAwaitSetup => Self::CombatAwaitSetup {},
            Phase::CombatReward => Self::CombatReward {},
            Phase::RestSite => Self::RestSite {},
            Phase::GameOver => Self::GameOver {},
            Phase::Chest => Self::Chest {},
            Phase::EventRoom => Self::EventRoom {},
            Phase::Shop => Self::Shop {},
        }
    }
}

#[pyclass(eq, hash, frozen, name = "SelectionKind")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PySelectionKind {
    All {},
    Single {},
    Random { count: u8 },
    Input { count: u8 },
}

impl From<SelectionKind> for PySelectionKind {
    fn from(s: SelectionKind) -> Self {
        match s {
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
    CardDiscard,
    CardRetain,
    CardSetup,
    CardNightmare,
    RoomSelect,
    CardRewardSelect,
    CardRewardSkip,
    RelicRewardSelect,
    RelicRewardSkip,
    RestSiteRest,
    RestSiteCardUpgrade,
    RoomSkip,
    ChestOpen,
}

impl PyActionType {
    fn from_discriminant(n: u8) -> Result<Self, String> {
        match n {
            0 => Ok(Self::CardPlay),
            1 => Ok(Self::EndTurn),
            2 => Ok(Self::CardDiscard),
            3 => Ok(Self::CardRetain),
            4 => Ok(Self::CardSetup),
            5 => Ok(Self::CardNightmare),
            6 => Ok(Self::RoomSelect),
            7 => Ok(Self::CardRewardSelect),
            8 => Ok(Self::CardRewardSkip),
            9 => Ok(Self::RelicRewardSelect),
            10 => Ok(Self::RelicRewardSkip),
            11 => Ok(Self::RestSiteRest),
            12 => Ok(Self::RestSiteCardUpgrade),
            13 => Ok(Self::RoomSkip),
            14 => Ok(Self::ChestOpen),
            _ => Err(format!("PyActionType: invalid discriminant {n}")),
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
}

#[pymethods]
impl PyAction {
    #[new]
    fn new(action_type: u8, idxs: Vec<usize>) -> PyResult<Self> {
        let action_type = PyActionType::from_discriminant(action_type)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(Self { action_type, idxs })
    }

    fn __repr__(&self) -> String {
        format!("PyAction({:?}, {:?})", self.action_type, self.idxs)
    }
}

pub fn to_internal_action(a: PyAction) -> Result<Action, String> {
    let i = &a.idxs;
    match a.action_type {
        PyActionType::CardPlay => match i.len() {
            1 => Ok(Action::CardPlay {
                idx_hand: i[0],
                idx_monster: None,
            }),
            2 => Ok(Action::CardPlay {
                idx_hand: i[0],
                idx_monster: Some(i[1]),
            }),
            n => Err(format!(
                "CardPlay expects [idx_hand] or [idx_hand, idx_monster], got {n} idxs"
            )),
        },
        PyActionType::EndTurn => match i.len() {
            0 => Ok(Action::EndTurn),
            n => Err(format!("EndTurn expects [], got {n} idxs")),
        },
        PyActionType::CardDiscard => Ok(Action::CardDiscard {
            indices_hand: i.clone(),
        }),
        PyActionType::CardRetain => Ok(Action::CardRetain {
            indices_hand: i.clone(),
        }),
        PyActionType::CardSetup => match i.len() {
            1 => Ok(Action::CardSetup { idx_hand: i[0] }),
            n => Err(format!("CardSetup expects [idx_hand], got {n} idxs")),
        },
        PyActionType::CardNightmare => match i.len() {
            1 => Ok(Action::CardNightmare { idx_hand: i[0] }),
            n => Err(format!("CardNightmare expects [idx_hand], got {n} idxs")),
        },
        PyActionType::RoomSelect => match i.len() {
            1 => Ok(Action::RoomSelect { idx_column: i[0] }),
            n => Err(format!("RoomSelect expects [idx_column], got {n} idxs")),
        },
        PyActionType::CardRewardSelect => match i.len() {
            1 => Ok(Action::CardRewardSelect { idx_reward: i[0] }),
            n => Err(format!(
                "CardRewardSelect expects [idx_reward], got {n} idxs"
            )),
        },
        PyActionType::CardRewardSkip => match i.len() {
            0 => Ok(Action::CardRewardSkip),
            n => Err(format!("CardRewardSkip expects [], got {n} idxs")),
        },
        PyActionType::RelicRewardSelect => match i.len() {
            1 => Ok(Action::RelicRewardSelect { idx_reward: i[0] }),
            n => Err(format!(
                "RelicRewardSelect expects [idx_reward], got {n} idxs"
            )),
        },
        PyActionType::RelicRewardSkip => match i.len() {
            0 => Ok(Action::RelicRewardSkip),
            n => Err(format!("RelicRewardSkip expects [], got {n} idxs")),
        },
        PyActionType::RestSiteRest => match i.len() {
            0 => Ok(Action::RestSiteRest),
            n => Err(format!("RestSiteRest expects [], got {n} idxs")),
        },
        PyActionType::RestSiteCardUpgrade => match i.len() {
            1 => Ok(Action::RestSiteCardUpgrade { idx_deck: i[0] }),
            n => Err(format!(
                "RestSiteCardUpgrade expects [idx_deck], got {n} idxs"
            )),
        },
        PyActionType::RoomSkip => match i.len() {
            0 => Ok(Action::RoomSkip),
            n => Err(format!("RoomSkip expects [], got {n} idxs")),
        },
        PyActionType::ChestOpen => match i.len() {
            0 => Ok(Action::ChestOpen),
            n => Err(format!("ChestOpen expects [], got {n} idxs")),
        },
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
        amount: u8,
        target: Option<PyTarget>,
    },
    CardAddToHand {
        card_name: String,
        count: u8,
        upgraded: bool,
        target: Option<PyTarget>,
    },
    CardDraw {
        count: u8,
        target: Option<PyTarget>,
    },
    DrawUpTo {
        amount: u8,
        target: Option<PyTarget>,
    },
    CardDiscard {
        target: Option<PyTarget>,
    },
    CalculatedGamble {
        target: Option<PyTarget>,
    },
}

fn snapshot_effect(effect: &Effect) -> PyEffect {
    let target = match effect.target {
        Target::Resolve {
            candidates,
            selection,
        } => Some(PyTarget {
            candidate_pool: candidates.into(),
            selection_kind: selection.into(),
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
        EffectKind::CalculatedGamble => PyEffect::CalculatedGamble { target },
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

#[pyclass(frozen, get_all, name = "Character")]
#[derive(Debug, Clone)]
pub struct PyCharacter {
    pub name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub gold: u16,
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
    fn from(i: Intent) -> Self {
        match i {
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
    pub character: PyCharacter,
    pub monsters: Vec<PyMonster>,
    pub deck: Vec<PyCard>,
    pub hand: Vec<PyCard>,
    pub pile_draw: Vec<PyCard>,
    pub pile_discard: Vec<PyCard>,
    pub pile_exhaust: Vec<PyCard>,
    pub rewards_card: Vec<PyCard>,
    pub relics: Vec<PyRelic>,
    pub rewards_relic: Vec<PyRelic>,
    pub energy: PyEnergy,
    pub map: PyMap,
    pub phase: PyPhase,
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
            Self::AllOutAttack => "All Out Attack",
            Self::Backflip => "Backflip",
            Self::Backstab => "Backstab",
            Self::Bane => "Bane",
            Self::BladeDance => "Blade Dance",
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
            Self::Finisher => "Finisher",
            Self::Flechettes => "Flechettes",
            Self::FlyingKnee => "Flying Knee",
            Self::Footwork => "Footwork",
            Self::GlassKnife => "Glass Knife",
            Self::GrandFinale => "Grand Finale",
            Self::HeelHook => "Heel Hook",
            Self::InfiniteBlades => "Infinite Blades",
            Self::LegSweep => "Leg Sweep",
            Self::Malaise => "Malaise",
            Self::MasterfulStab => "Masterful Stab",
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
    let this_turn_discards = state.this_turn_discards;
    let this_combat_damage_instances_taken = state.this_combat_damage_instances_taken;
    let energy_current = state.energy.current;
    let entangled = modifier_has(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Entangled,
    );
    let card = |id_card: usize| {
        snapshot_card(
            &state.entities[id_card],
            &state.id_pile_draw,
            this_turn_discards,
            this_combat_damage_instances_taken,
            energy_current,
            entangled,
        )
    };
    let relic = |id_relic: usize| snapshot_relic(&state.entities[id_relic]);
    PyGameState {
        character: snapshot_character(state),
        monsters: snapshot_monsters(state),
        deck: state.id_deck.iter().copied().map(card).collect(),
        hand: state.id_hand.iter().copied().map(card).collect(),
        pile_draw: state.id_pile_draw.iter().copied().map(card).collect(),
        pile_discard: state.id_pile_discard.iter().copied().map(card).collect(),
        pile_exhaust: state.id_pile_exhaust.iter().copied().map(card).collect(),
        rewards_card: state.id_card_rewards.iter().copied().map(card).collect(),
        relics: iter_owned_relics(&state.id_relics)
            .map(|(_name, id)| snapshot_relic(&state.entities[id]))
            .collect(),
        rewards_relic: state.id_relic_rewards.iter().copied().map(relic).collect(),
        energy: PyEnergy {
            current: state.energy.current,
            max: state.energy.max,
        },
        map: snapshot_map(state),
        phase: state.phase.into(),
    }
}

fn snapshot_relic(e: &Entity) -> PyRelic {
    PyRelic {
        name: e.relic_name.into(),
        tier: e.relic_tier.into(),
        counter: e.relic_counter,
        used_up: e.relic_used_up,
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
    let mut buf_alive = [0usize; MAX_MONSTERS];
    let n = fill_alive_monster_ids(state, &mut buf_alive);
    buf_alive[..n]
        .iter()
        .map(|&id_monster| {
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
        let kind = ModifierKind::from_u8(idx as u8);
        out.push(PyModifier {
            kind: kind.into(),
            stacks: mods.stacks[idx],
            stacks_max: stacks_max_for(kind),
        });
    }
    out
}

fn snapshot_card(
    card: &Entity,
    id_pile_draw: &[usize],
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
    entangled: bool,
) -> PyCard {
    let restriction_ok = is_play_restriction_satisfied(card.card_play_restriction, id_pile_draw);
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
        cost: card_effective_cost(
            card,
            this_turn_discards,
            this_combat_damage_instances_taken,
            energy_current,
        ),
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
        requires_target: card.card_requires_target,
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
