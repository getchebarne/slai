// FFI boundary: every #[pyclass] type lives here. Internal engine modules
// must not import pyo3
//
// Naming: structs that snapshot internal engine state (GameState, Card, ...)
// take the bare name. Where the bare name would collide with an internal type
// at the Rust level (engine `game::GameState`, `entity::Intent`), we alias
// the internal import below

use pyo3::prelude::*;

use crate::action::Action as InternalAction;
use crate::consts::{HEXAGHOST_DIVIDER_HITS, MAP_HEIGHT, MAX_MONSTERS};
use crate::monsters::hexaghost;
use crate::effect::{
    CandidatePool as InternalCandidatePool, Effect as InternalEffect, EffectKind,
    SelectionKind, Target as InternalTarget,
};
use crate::entity::{
    CardCostKind as InternalCardCostKind, Entity, Intent as InternalIntent, card_effective_cost,
    is_play_restriction_satisfied,
};
use crate::map::edge_indices;
use crate::modifier::{
    ModifierKind as InternalModifierKind, Modifiers, modifier_has, modifier_stacks,
    stacks_max_for as internal_stacks_max_for,
};
use crate::game::{GameState as InternalGameState, Location};
use crate::types::{
    CardColor as InternalCardColor, CardKind as InternalCardKind,
    CardName as InternalCardName, CardRarity as InternalCardRarity, MonsterEncounter,
    MonsterName as InternalMonsterName, Phase as InternalPhase,
    RelicName as InternalRelicName, RelicTier as InternalRelicTier, RoomKind as InternalRoomKind,
};
use crate::utils::{fill_alive_monster_ids, scale_attack_damage};

#[pyclass(eq, eq_int, hash, frozen, name = "CardKind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardKind {
    Attack,
    Skill,
    Power,
    Curse,
    Status,
}

impl From<InternalCardKind> for CardKind {
    fn from(k: InternalCardKind) -> Self {
        match k {
            InternalCardKind::Attack => Self::Attack,
            InternalCardKind::Skill => Self::Skill,
            InternalCardKind::Power => Self::Power,
            InternalCardKind::Curse => Self::Curse,
            InternalCardKind::Status => Self::Status,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "CardColor")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardColor {
    Green,
    Colorless,
    Curse,
}

impl From<InternalCardColor> for CardColor {
    fn from(c: InternalCardColor) -> Self {
        match c {
            InternalCardColor::Green => Self::Green,
            InternalCardColor::Colorless => Self::Colorless,
            InternalCardColor::Curse => Self::Curse,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "CardRarity")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

impl From<InternalCardRarity> for CardRarity {
    fn from(r: InternalCardRarity) -> Self {
        match r {
            InternalCardRarity::Basic => Self::Basic,
            InternalCardRarity::Common => Self::Common,
            InternalCardRarity::Uncommon => Self::Uncommon,
            InternalCardRarity::Rare => Self::Rare,
            InternalCardRarity::Special => Self::Special,
            InternalCardRarity::Curse => Self::Curse,
        }
    }
}

#[pyclass(eq, hash, frozen, name = "CardCostKind")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CardCostKind {
    Fixed {},
    MinusDiscardsThisTurn {},
    GrowsOnDamageInstanceTaken {},
    XCost { offset: i8 },
}

impl From<InternalCardCostKind> for CardCostKind {
    fn from(k: InternalCardCostKind) -> Self {
        match k {
            InternalCardCostKind::Fixed => Self::Fixed {},
            InternalCardCostKind::MinusDiscardsThisTurn => Self::MinusDiscardsThisTurn {},
            InternalCardCostKind::GrowsOnDamageInstanceTaken => Self::GrowsOnDamageInstanceTaken {},
            InternalCardCostKind::XCost { offset } => Self::XCost { offset },
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "RoomKind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoomKind {
    CombatMonster,
    CombatElite,
    CombatBoss,
    RestSite,
}

impl From<InternalRoomKind> for RoomKind {
    fn from(r: InternalRoomKind) -> Self {
        match r {
            InternalRoomKind::CombatMonster => Self::CombatMonster,
            InternalRoomKind::CombatElite => Self::CombatElite,
            InternalRoomKind::CombatBoss => Self::CombatBoss,
            InternalRoomKind::RestSite => Self::RestSite,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "RelicName")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelicName {
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
}

impl From<InternalRelicName> for RelicName {
    fn from(n: InternalRelicName) -> Self {
        match n {
            InternalRelicName::SnakeRing => Self::SnakeRing,
            InternalRelicName::Akabeko => Self::Akabeko,
            InternalRelicName::Anchor => Self::Anchor,
            InternalRelicName::BagOfMarbles => Self::BagOfMarbles,
            InternalRelicName::BagOfPreparation => Self::BagOfPreparation,
            InternalRelicName::BloodVial => Self::BloodVial,
            InternalRelicName::BronzeScales => Self::BronzeScales,
            InternalRelicName::Kunai => Self::Kunai,
            InternalRelicName::NinjaScroll => Self::NinjaScroll,
            InternalRelicName::OddlySmoothStone => Self::OddlySmoothStone,
            InternalRelicName::Shuriken => Self::Shuriken,
            InternalRelicName::ThreadAndNeedle => Self::ThreadAndNeedle,
            InternalRelicName::TwistedFunnel => Self::TwistedFunnel,
            InternalRelicName::Vajra => Self::Vajra,
        }
    }
}

impl From<RelicName> for InternalRelicName {
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
        }
    }
}

// CardName mirror: typed enum so Python can index a one-hot directly
// instead of parsing display strings (which include "+" suffix on
// upgrades and have spaces). 78 variants — keep in lockstep with
// `crate::types::CardName`.
#[pyclass(eq, eq_int, hash, frozen, name = "CardName")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardName {
    AThousandCuts, Accuracy, Acrobatics, Adrenaline, AfterImage, AllOutAttack,
    Backflip, Backstab, Bane, BladeDance, Blur, BouncingFlask, BulletTime,
    Burn, Burst, CalculatedGamble, Caltrops, Catalyst, Choke, CloakAndDagger,
    Concentrate, CorpseExplosion, CripplingPoison, DaggerSpray, DaggerThrow,
    Dash, Dazed, DeadlyPoison, Defend, Deflect, DieDieDie, Distraction,
    DodgeAndRoll, Doppelganger, EndlessAgony, Envenom, EscapePlan,
    Eviscerate, Expertise, Finisher, Flechettes, FlyingKnee, Footwork,
    GlassKnife, GrandFinale, HeelHook, InfiniteBlades, LegSweep, Malaise,
    MasterfulStab, Neutralize, Nightmare, NoxiousFumes, Outmaneuver,
    PhantasmalKiller, PiercingWail, PoisonedStab, Predator, Prepared,
    QuickSlash, Reflex, RiddleWithHoles, Setup, Shiv, Skewer, Slice, Slimed,
    SneakyStrike, StormOfSteel, Strike, SuckerPunch, Survivor, Tactician,
    Terror, ToolsOfTheTrade, Unload, WellLaidPlans, WraithForm,
}

impl From<InternalCardName> for CardName {
    fn from(n: InternalCardName) -> Self {
        // Variants are 1:1 by name — uses repr(u8) on both sides for
        // a single transmute would work, but the explicit match keeps
        // the layout coupling honest if either enum drifts.
        match n {
            InternalCardName::AThousandCuts => Self::AThousandCuts,
            InternalCardName::Accuracy => Self::Accuracy,
            InternalCardName::Acrobatics => Self::Acrobatics,
            InternalCardName::Adrenaline => Self::Adrenaline,
            InternalCardName::AfterImage => Self::AfterImage,
            InternalCardName::AllOutAttack => Self::AllOutAttack,
            InternalCardName::Backflip => Self::Backflip,
            InternalCardName::Backstab => Self::Backstab,
            InternalCardName::Bane => Self::Bane,
            InternalCardName::BladeDance => Self::BladeDance,
            InternalCardName::Blur => Self::Blur,
            InternalCardName::BouncingFlask => Self::BouncingFlask,
            InternalCardName::BulletTime => Self::BulletTime,
            InternalCardName::Burn => Self::Burn,
            InternalCardName::Burst => Self::Burst,
            InternalCardName::CalculatedGamble => Self::CalculatedGamble,
            InternalCardName::Caltrops => Self::Caltrops,
            InternalCardName::Catalyst => Self::Catalyst,
            InternalCardName::Choke => Self::Choke,
            InternalCardName::CloakAndDagger => Self::CloakAndDagger,
            InternalCardName::Concentrate => Self::Concentrate,
            InternalCardName::CorpseExplosion => Self::CorpseExplosion,
            InternalCardName::CripplingPoison => Self::CripplingPoison,
            InternalCardName::DaggerSpray => Self::DaggerSpray,
            InternalCardName::DaggerThrow => Self::DaggerThrow,
            InternalCardName::Dash => Self::Dash,
            InternalCardName::Dazed => Self::Dazed,
            InternalCardName::DeadlyPoison => Self::DeadlyPoison,
            InternalCardName::Defend => Self::Defend,
            InternalCardName::Deflect => Self::Deflect,
            InternalCardName::DieDieDie => Self::DieDieDie,
            InternalCardName::Distraction => Self::Distraction,
            InternalCardName::DodgeAndRoll => Self::DodgeAndRoll,
            InternalCardName::Doppelganger => Self::Doppelganger,
            InternalCardName::EndlessAgony => Self::EndlessAgony,
            InternalCardName::Envenom => Self::Envenom,
            InternalCardName::EscapePlan => Self::EscapePlan,
            InternalCardName::Eviscerate => Self::Eviscerate,
            InternalCardName::Expertise => Self::Expertise,
            InternalCardName::Finisher => Self::Finisher,
            InternalCardName::Flechettes => Self::Flechettes,
            InternalCardName::FlyingKnee => Self::FlyingKnee,
            InternalCardName::Footwork => Self::Footwork,
            InternalCardName::GlassKnife => Self::GlassKnife,
            InternalCardName::GrandFinale => Self::GrandFinale,
            InternalCardName::HeelHook => Self::HeelHook,
            InternalCardName::InfiniteBlades => Self::InfiniteBlades,
            InternalCardName::LegSweep => Self::LegSweep,
            InternalCardName::Malaise => Self::Malaise,
            InternalCardName::MasterfulStab => Self::MasterfulStab,
            InternalCardName::Neutralize => Self::Neutralize,
            InternalCardName::Nightmare => Self::Nightmare,
            InternalCardName::NoxiousFumes => Self::NoxiousFumes,
            InternalCardName::Outmaneuver => Self::Outmaneuver,
            InternalCardName::PhantasmalKiller => Self::PhantasmalKiller,
            InternalCardName::PiercingWail => Self::PiercingWail,
            InternalCardName::PoisonedStab => Self::PoisonedStab,
            InternalCardName::Predator => Self::Predator,
            InternalCardName::Prepared => Self::Prepared,
            InternalCardName::QuickSlash => Self::QuickSlash,
            InternalCardName::Reflex => Self::Reflex,
            InternalCardName::RiddleWithHoles => Self::RiddleWithHoles,
            InternalCardName::Setup => Self::Setup,
            InternalCardName::Shiv => Self::Shiv,
            InternalCardName::Skewer => Self::Skewer,
            InternalCardName::Slice => Self::Slice,
            InternalCardName::Slimed => Self::Slimed,
            InternalCardName::SneakyStrike => Self::SneakyStrike,
            InternalCardName::StormOfSteel => Self::StormOfSteel,
            InternalCardName::Strike => Self::Strike,
            InternalCardName::SuckerPunch => Self::SuckerPunch,
            InternalCardName::Survivor => Self::Survivor,
            InternalCardName::Tactician => Self::Tactician,
            InternalCardName::Terror => Self::Terror,
            InternalCardName::ToolsOfTheTrade => Self::ToolsOfTheTrade,
            InternalCardName::Unload => Self::Unload,
            InternalCardName::WellLaidPlans => Self::WellLaidPlans,
            InternalCardName::WraithForm => Self::WraithForm,
        }
    }
}

// MonsterName mirror — 25 variants, same shape as CardName.
#[pyclass(eq, eq_int, hash, frozen, name = "MonsterName")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterName {
    Cultist, FungiBeast, GremlinFat, GremlinNob, GremlinThief,
    GremlinTsundere, GremlinWarrior, GremlinWizard, Hexaghost, JawWorm,
    Lagavulin, Looter, LouseDefensive, LouseNormal, Sentry, SlaverBlue,
    SlaverRed, SlimeAcidLarge, SlimeAcidMedium, SlimeAcidSmall, SlimeBoss,
    SlimeSpikeLarge, SlimeSpikeMedium, SlimeSpikeSmall, TheGuardian,
}

impl From<InternalMonsterName> for MonsterName {
    fn from(n: InternalMonsterName) -> Self {
        match n {
            InternalMonsterName::Cultist => Self::Cultist,
            InternalMonsterName::FungiBeast => Self::FungiBeast,
            InternalMonsterName::GremlinFat => Self::GremlinFat,
            InternalMonsterName::GremlinNob => Self::GremlinNob,
            InternalMonsterName::GremlinThief => Self::GremlinThief,
            InternalMonsterName::GremlinTsundere => Self::GremlinTsundere,
            InternalMonsterName::GremlinWarrior => Self::GremlinWarrior,
            InternalMonsterName::GremlinWizard => Self::GremlinWizard,
            InternalMonsterName::Hexaghost => Self::Hexaghost,
            InternalMonsterName::JawWorm => Self::JawWorm,
            InternalMonsterName::Lagavulin => Self::Lagavulin,
            InternalMonsterName::Looter => Self::Looter,
            InternalMonsterName::LouseDefensive => Self::LouseDefensive,
            InternalMonsterName::LouseNormal => Self::LouseNormal,
            InternalMonsterName::Sentry => Self::Sentry,
            InternalMonsterName::SlaverBlue => Self::SlaverBlue,
            InternalMonsterName::SlaverRed => Self::SlaverRed,
            InternalMonsterName::SlimeAcidLarge => Self::SlimeAcidLarge,
            InternalMonsterName::SlimeAcidMedium => Self::SlimeAcidMedium,
            InternalMonsterName::SlimeAcidSmall => Self::SlimeAcidSmall,
            InternalMonsterName::SlimeBoss => Self::SlimeBoss,
            InternalMonsterName::SlimeSpikeLarge => Self::SlimeSpikeLarge,
            InternalMonsterName::SlimeSpikeMedium => Self::SlimeSpikeMedium,
            InternalMonsterName::SlimeSpikeSmall => Self::SlimeSpikeSmall,
            InternalMonsterName::TheGuardian => Self::TheGuardian,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "RelicTier")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelicTier {
    Starter,
    Common,
    Uncommon,
    Rare,
    Boss,
    Shop,
    Special,
}

impl From<InternalRelicTier> for RelicTier {
    fn from(t: InternalRelicTier) -> Self {
        match t {
            InternalRelicTier::Starter => Self::Starter,
            InternalRelicTier::Common => Self::Common,
            InternalRelicTier::Uncommon => Self::Uncommon,
            InternalRelicTier::Rare => Self::Rare,
            InternalRelicTier::Boss => Self::Boss,
            InternalRelicTier::Shop => Self::Shop,
            InternalRelicTier::Special => Self::Special,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "ModifierKind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierKind {
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

impl From<InternalModifierKind> for ModifierKind {
    fn from(k: InternalModifierKind) -> Self {
        match k {
            InternalModifierKind::Accuracy => Self::Accuracy,
            InternalModifierKind::AfterImage => Self::AfterImage,
            InternalModifierKind::Angry => Self::Angry,
            InternalModifierKind::Artifact => Self::Artifact,
            InternalModifierKind::Asleep => Self::Asleep,
            InternalModifierKind::Blur => Self::Blur,
            InternalModifierKind::Burst => Self::Burst,
            InternalModifierKind::Choke => Self::Choke,
            InternalModifierKind::CorpseExplosion => Self::CorpseExplosion,
            InternalModifierKind::CurlUp => Self::CurlUp,
            InternalModifierKind::Dexterity => Self::Dexterity,
            InternalModifierKind::DoubleDamage => Self::DoubleDamage,
            InternalModifierKind::DrawCardNextTurn => Self::DrawCardNextTurn,
            InternalModifierKind::Enrage => Self::Enrage,
            InternalModifierKind::Entangled => Self::Entangled,
            InternalModifierKind::Envenom => Self::Envenom,
            InternalModifierKind::Frail => Self::Frail,
            InternalModifierKind::InfiniteBlades => Self::InfiniteBlades,
            InternalModifierKind::Intangible => Self::Intangible,
            InternalModifierKind::Metallicize => Self::Metallicize,
            InternalModifierKind::ModeShift => Self::ModeShift,
            InternalModifierKind::NextTurnBlock => Self::NextTurnBlock,
            InternalModifierKind::NextTurnEnergy => Self::NextTurnEnergy,
            InternalModifierKind::NoDraw => Self::NoDraw,
            InternalModifierKind::NoxiousFumes => Self::NoxiousFumes,
            InternalModifierKind::Phantasmal => Self::Phantasmal,
            InternalModifierKind::PlatedArmor => Self::PlatedArmor,
            InternalModifierKind::Poison => Self::Poison,
            InternalModifierKind::Retain => Self::Retain,
            InternalModifierKind::Ritual => Self::Ritual,
            InternalModifierKind::Shackled => Self::Shackled,
            InternalModifierKind::SharpHide => Self::SharpHide,
            InternalModifierKind::Splittable => Self::Splittable,
            InternalModifierKind::SporeCloud => Self::SporeCloud,
            InternalModifierKind::Strength => Self::Strength,
            InternalModifierKind::Thievery => Self::Thievery,
            InternalModifierKind::Thorns => Self::Thorns,
            InternalModifierKind::ThousandCuts => Self::ThousandCuts,
            InternalModifierKind::ToolsOfTheTrade => Self::ToolsOfTheTrade,
            InternalModifierKind::Vigor => Self::Vigor,
            InternalModifierKind::Vulnerable => Self::Vulnerable,
            InternalModifierKind::Weak => Self::Weak,
            InternalModifierKind::WraithForm => Self::WraithForm,
        }
    }
}

#[pyclass(eq, eq_int, hash, frozen, name = "CandidatePool")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CandidatePool {
    Hand,
    CardTarget,
    Character,
    Monsters,
    OtherMonsters,
    Source,
    NextRowRooms,
    CardRewardPool,
}

impl From<InternalCandidatePool> for CandidatePool {
    fn from(c: InternalCandidatePool) -> Self {
        match c {
            InternalCandidatePool::Hand => Self::Hand,
            InternalCandidatePool::CardTarget => Self::CardTarget,
            InternalCandidatePool::Character => Self::Character,
            InternalCandidatePool::Monsters => Self::Monsters,
            InternalCandidatePool::OtherMonsters => Self::OtherMonsters,
            InternalCandidatePool::Source => Self::Source,
            InternalCandidatePool::NextRowRooms => Self::NextRowRooms,
            InternalCandidatePool::CardRewardPool => Self::CardRewardPool,
        }
    }
}

#[pyclass(eq, hash, frozen, name = "Phase")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Phase {
    Map {},
    CombatDefault {},
    CombatAwaitDiscard { num: u8 },
    CombatAwaitNightmare {},
    CombatAwaitRetain { num: u8 },
    CombatAwaitSetup {},
    CombatReward {},
    RestSite {},
    GameOver {},
}

impl From<InternalPhase> for Phase {
    fn from(p: InternalPhase) -> Self {
        match p {
            InternalPhase::Map => Self::Map {},
            InternalPhase::CombatDefault => Self::CombatDefault {},
            InternalPhase::CombatAwaitDiscard { num } => Self::CombatAwaitDiscard { num },
            InternalPhase::CombatAwaitNightmare => Self::CombatAwaitNightmare {},
            InternalPhase::CombatAwaitRetain { num } => Self::CombatAwaitRetain { num },
            InternalPhase::CombatAwaitSetup => Self::CombatAwaitSetup {},
            InternalPhase::CombatReward => Self::CombatReward {},
            InternalPhase::RestSite => Self::RestSite {},
            InternalPhase::GameOver => Self::GameOver {},
        }
    }
}

#[pyclass(eq, hash, frozen, name = "Selection")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selection {
    All {},
    Single {},
    Random { count: u8 },
    Input { count: u8 },
}

impl From<SelectionKind> for Selection {
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
pub struct Target {
    pub candidates: CandidatePool,
    pub selection: Selection,
}

// `ActionType` is the discriminant for the flat `Action` struct below.
// Per-action argument schemas (names + meanings) live next to ACTION_SPECS
// in `python/slai/__init__.py`; the arity match in TryFrom below must
// stay in sync with that table.
#[pyclass(eq, eq_int, hash, frozen, name = "ActionType")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
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
}

impl ActionType {
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
            _ => Err(format!("ActionType: invalid discriminant {n}")),
        }
    }
}

// Flat heterogeneous action: a discriminant plus a positional `indices`
// list. Mirrors PySC2's `FunctionCall(function_id, arguments)`. Each
// position's meaning depends on `action_type` — see `ACTION_SPECS` in
// `python/slai/__init__.py` for the per-type schema.
#[pyclass(eq, hash, frozen, name = "Action")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Action {
    #[pyo3(get)]
    pub action_type: ActionType,
    #[pyo3(get)]
    pub indices: Vec<usize>,
}

#[pymethods]
impl Action {
    // Accept the discriminant as a u8 so users can pass either the PyO3
    // `ActionType` (it has __int__) or the Python IntEnum shim (it is an
    // int). Both go through the same numeric conversion path.
    #[new]
    fn new(action_type: u8, indices: Vec<usize>) -> PyResult<Self> {
        let action_type = ActionType::from_discriminant(action_type)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(Self { action_type, indices })
    }

    fn __repr__(&self) -> String {
        format!("Action({:?}, {:?})", self.action_type, self.indices)
    }
}

impl TryFrom<Action> for InternalAction {
    type Error = String;
    fn try_from(a: Action) -> Result<Self, Self::Error> {
        let i = &a.indices;
        match a.action_type {
            ActionType::CardPlay => match i.len() {
                1 => Ok(InternalAction::CardPlay {
                    idx_hand: i[0],
                    idx_monster: None,
                }),
                2 => Ok(InternalAction::CardPlay {
                    idx_hand: i[0],
                    idx_monster: Some(i[1]),
                }),
                n => Err(format!(
                    "CardPlay expects [idx_hand] or [idx_hand, idx_monster], got {n} indices"
                )),
            },
            ActionType::EndTurn => match i.len() {
                0 => Ok(InternalAction::EndTurn),
                n => Err(format!("EndTurn expects [], got {n} indices")),
            },
            ActionType::CardDiscard => Ok(InternalAction::CardDiscard {
                indices_hand: i.clone(),
            }),
            ActionType::CardRetain => Ok(InternalAction::CardRetain {
                indices_hand: i.clone(),
            }),
            ActionType::CardSetup => match i.len() {
                1 => Ok(InternalAction::CardSetup { idx_hand: i[0] }),
                n => Err(format!("CardSetup expects [idx_hand], got {n} indices")),
            },
            ActionType::CardNightmare => match i.len() {
                1 => Ok(InternalAction::CardNightmare { idx_hand: i[0] }),
                n => Err(format!("CardNightmare expects [idx_hand], got {n} indices")),
            },
            ActionType::RoomSelect => match i.len() {
                1 => Ok(InternalAction::RoomSelect { idx_column: i[0] }),
                n => Err(format!("RoomSelect expects [idx_column], got {n} indices")),
            },
            ActionType::CardRewardSelect => match i.len() {
                1 => Ok(InternalAction::CardRewardSelect { idx_reward: i[0] }),
                n => Err(format!(
                    "CardRewardSelect expects [idx_reward], got {n} indices"
                )),
            },
            ActionType::CardRewardSkip => match i.len() {
                0 => Ok(InternalAction::CardRewardSkip),
                n => Err(format!("CardRewardSkip expects [], got {n} indices")),
            },
            ActionType::RelicRewardSelect => match i.len() {
                1 => Ok(InternalAction::RelicRewardSelect { idx_reward: i[0] }),
                n => Err(format!(
                    "RelicRewardSelect expects [idx_reward], got {n} indices"
                )),
            },
            ActionType::RelicRewardSkip => match i.len() {
                0 => Ok(InternalAction::RelicRewardSkip),
                n => Err(format!("RelicRewardSkip expects [], got {n} indices")),
            },
            ActionType::RestSiteRest => match i.len() {
                0 => Ok(InternalAction::RestSiteRest),
                n => Err(format!("RestSiteRest expects [], got {n} indices")),
            },
            ActionType::RestSiteCardUpgrade => match i.len() {
                1 => Ok(InternalAction::RestSiteCardUpgrade { idx_deck: i[0] }),
                n => Err(format!(
                    "RestSiteCardUpgrade expects [idx_deck], got {n} indices"
                )),
            },
        }
    }
}

// `Effect` mirrors only the EffectKind variants that appear in static
// card/monster definitions (~9 of EffectKind's ~33). `target` is None for
// effects with no resolution (e.g. CardDraw, EnergyGain on the player)
// `from_internal` panics on EffectKind variants that should never reach
// the view layer

#[pyclass(eq, hash, frozen, name = "Effect")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Effect {
    DamagePhysical {
        amount: u16,
        target: Option<Target>,
    },
    DamagePhysicalIfPoisoned {
        amount: u16,
        target: Option<Target>,
    },
    HeelHookProc {
        target: Option<Target>,
    },
    EscapePlanCheck {
        block: u16,
        target: Option<Target>,
    },
    GlassKnifeDecay {
        delta: i16,
        target: Option<Target>,
    },
    CardSetupPick {
        target: Option<Target>,
    },
    CardNightmarePick {
        target: Option<Target>,
    },
    DistractionAdd {
        target: Option<Target>,
    },
    SetCostOverride {
        amount: u8,
        target: Option<Target>,
    },
    FinisherDamage {
        damage: u16,
        target: Option<Target>,
    },
    FlechettesDamage {
        damage: u16,
        target: Option<Target>,
    },
    UnloadDiscard {
        target: Option<Target>,
    },
    StormOfSteelProc {
        upgraded: bool,
        target: Option<Target>,
    },
    SneakyStrikeProc {
        energy: u8,
        target: Option<Target>,
    },
    BlockGain {
        amount: u16,
        target: Option<Target>,
    },
    ModifierGain {
        kind: ModifierKind,
        stacks: i16,
        target: Option<Target>,
    },
    ModifierMultiply {
        kind: ModifierKind,
        factor: u8,
        target: Option<Target>,
    },
    ModifierRemove {
        kind: ModifierKind,
        target: Option<Target>,
    },
    EnergyGain {
        amount: u8,
        target: Option<Target>,
    },
    CardAddToHand {
        card_name: String,
        count: u8,
        upgraded: bool,
        target: Option<Target>,
    },
    CardDraw {
        count: u8,
        target: Option<Target>,
    },
    DrawUpTo {
        target: u8,
        target_field: Option<Target>,
    },
    CardDiscard {
        target: Option<Target>,
    },
    CalculatedGamble {
        target: Option<Target>,
    },
}

impl Effect {
    fn from_internal(effect: &InternalEffect) -> Self {
        let target = match effect.target {
            InternalTarget::Resolve {
                candidates,
                selection,
            } => Some(Target {
                candidates: candidates.into(),
                selection: selection.into(),
            }),
            InternalTarget::Direct(None) => None,
            InternalTarget::Direct(Some(_)) => panic!(
                "Effect::from_internal: unexpected Direct(Some) on static card effect: {:?}",
                effect,
            ),
        };
        match effect.kind {
            EffectKind::DamagePhysical { amount } => Self::DamagePhysical { amount, target },
            EffectKind::DamagePhysicalIfPoisoned { amount } => {
                Self::DamagePhysicalIfPoisoned { amount, target }
            }
            EffectKind::HeelHookProc => Self::HeelHookProc { target },
            EffectKind::EscapePlanCheck { block } => Self::EscapePlanCheck { block, target },
            EffectKind::GlassKnifeDecay { delta } => Self::GlassKnifeDecay { delta, target },
            EffectKind::CardSetupPick => Self::CardSetupPick { target },
            EffectKind::CardNightmarePick => Self::CardNightmarePick { target },
            EffectKind::DistractionAdd => Self::DistractionAdd { target },
            EffectKind::SetCostOverride { amount } => Self::SetCostOverride { amount, target },
            EffectKind::FinisherDamage { damage } => Self::FinisherDamage { damage, target },
            EffectKind::FlechettesDamage { damage } => Self::FlechettesDamage { damage, target },
            EffectKind::UnloadDiscard => Self::UnloadDiscard { target },
            EffectKind::StormOfSteelProc { upgraded } => {
                Self::StormOfSteelProc { upgraded, target }
            }
            EffectKind::SneakyStrikeProc { energy } => Self::SneakyStrikeProc { energy, target },
            EffectKind::BlockGain { amount } => Self::BlockGain { amount, target },
            EffectKind::ModifierGain { kind, stacks } => Self::ModifierGain {
                kind: kind.into(),
                stacks,
                target,
            },
            EffectKind::ModifierMultiply { kind, factor } => Self::ModifierMultiply {
                kind: kind.into(),
                factor,
                target,
            },
            EffectKind::ModifierRemove { kind } => Self::ModifierRemove {
                kind: kind.into(),
                target,
            },
            EffectKind::EnergyGain { amount } => Self::EnergyGain { amount, target },
            EffectKind::CardAddToHand {
                card_name,
                count,
                upgraded,
            } => Self::CardAddToHand {
                card_name: card_name.as_str().to_string(),
                count,
                upgraded,
                target,
            },
            EffectKind::CardDraw { count } => Self::CardDraw { count, target },
            EffectKind::DrawUpTo { target: n } => Self::DrawUpTo {
                target: n,
                target_field: target,
            },
            EffectKind::CardDiscard { source: _ } => Self::CardDiscard { target },
            EffectKind::CalculatedGamble => Self::CalculatedGamble { target },
            other => unreachable!(
                "Effect::from_internal: unexpected EffectKind on static card effect: {:?}",
                other
            ),
        }
    }
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Card {
    /// Display name (includes "+" suffix for upgrades, has spaces).
    pub name: String,
    /// Canonical enum slot — stable across upgrades, suitable for one-hot.
    pub card_name: CardName,
    pub kind: CardKind,
    pub color: CardColor,
    pub rarity: CardRarity,
    /// Effective cost right now (post free-to-play, post BulletTime override,
    /// post dynamic-cost variant). For X-cost cards this is `energy.current`.
    pub cost: u8,
    /// Static base cost (the deck-instance value, before any modifiers).
    /// Distinct from `cost` for dynamic-cost cards (Eviscerate, MasterfulStab,
    /// X-cost). Use this to recover the un-discounted card cost.
    pub base_cost: u8,
    /// Tag describing how `cost` is derived. Lets the agent reason about
    /// X-cost / "discounted from base" / "growing this combat" without
    /// inferring it from card identity.
    pub cost_kind: CardCostKind,
    pub upgraded: bool,
    pub exhaust: bool,
    pub ethereal: bool,
    pub innate: bool,
    pub requires_target: bool,
    pub retain: bool,
    /// Per-instance "free to play once" flag (set by Setup, Distraction).
    /// When true, the next play of this card instance ignores energy cost.
    pub free_to_play_once: bool,
    /// Whether this card can be played given the current game state.
    /// Combines its static `card_play_restriction` with the relevant state
    /// (currently: `id_pile_draw` for the DrawPileEmpty restriction).
    /// Energy cost is NOT factored in — clients should also check
    /// `card.cost <= energy.current` before offering it as a legal action.
    pub playable: bool,
    pub effects: Vec<Effect>,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Modifier {
    pub kind: ModifierKind,
    pub stacks: i16,
}

#[pymethods]
impl Modifier {
    /// Per-`ModifierKind` stack ceiling from the engine's `MODIFIER_DEFS`.
    /// Useful for normalizing stacks before feeding to ML encoders.
    /// Soft caps (e.g. 999) are common — clamp again on the consumer side
    /// if a tighter normalization range is wanted.
    ///
    /// Accepts the discriminant as `u8` so users can pass either the
    /// PyO3 `ModifierKind` (it has __int__) or the Python IntEnum shim
    /// (it is an int).
    #[staticmethod]
    fn stacks_max_for(kind: u8) -> PyResult<i16> {
        if (kind as usize) >= crate::modifier::MODIFIER_COUNT {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "ModifierKind: invalid discriminant {kind}"
            )));
        }
        Ok(internal_stacks_max_for(InternalModifierKind::from_u8(kind)))
    }
}

// Reverse conversion needed by Modifier::stacks_max_for. Variants are 1:1
// by name with the internal enum.
impl From<ModifierKind> for InternalModifierKind {
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

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Relic {
    pub name: RelicName,
    pub tier: RelicTier,
    pub counter: i16,
    pub used_up: bool,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<Modifier>,
    pub character_reward_roll_offset: i8,
    pub gold: u16,
}

#[pyclass(eq, eq_int, hash, frozen, name = "IntentKind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntentKind {
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

impl From<InternalIntent> for IntentKind {
    fn from(i: InternalIntent) -> Self {
        match i {
            InternalIntent::Attack { .. } => Self::Attack,
            InternalIntent::AttackBlock { .. } => Self::AttackBlock,
            InternalIntent::AttackBuff { .. } => Self::AttackBuff,
            InternalIntent::AttackDebuff { .. } => Self::AttackDebuff,
            InternalIntent::Block => Self::Block,
            InternalIntent::BlockBuff => Self::BlockBuff,
            InternalIntent::Buff => Self::Buff,
            InternalIntent::Debuff => Self::Debuff,
            InternalIntent::DebuffPowerful => Self::DebuffPowerful,
            InternalIntent::Escape => Self::Escape,
            InternalIntent::Sleep => Self::Sleep,
            InternalIntent::Stunned => Self::Stunned,
            InternalIntent::Unknown => Self::Unknown,
        }
    }
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Intent {
    pub kind: IntentKind,
    pub damage: Option<u16>,
    pub instances: Option<u8>,
    pub block: bool,
    pub buff: bool,
    pub debuff: bool,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Monster {
    /// Display name (e.g. "Acid Slime (L)", "Gremlin Nob").
    pub name: String,
    /// Canonical enum slot — suitable for one-hot.
    pub monster_name: MonsterName,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<Modifier>,
    pub intent: Intent,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
}

#[pyclass(name = "Room", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct MapNode {
    pub room_kind: RoomKind,
    pub edges: Vec<usize>,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Map {
    pub rooms: Vec<Vec<Option<MapNode>>>,
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
    pub boss_name: String,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct GameState {
    pub character: Character,
    pub monsters: Vec<Monster>,
    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    pub pile_draw: Vec<Card>,
    pub pile_discard: Vec<Card>,
    pub pile_exhaust: Vec<Card>,
    pub card_rewards: Vec<Card>,
    pub relics: Vec<Relic>,
    pub relic_rewards: Vec<Relic>,
    pub energy: Energy,
    pub map: Map,
    pub phase: Phase,
}

impl InternalCardName {
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
        }
    }
}

impl InternalMonsterName {
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

pub fn build_view(state: &InternalGameState) -> GameState {
    let this_turn_discards = state.this_turn_discards;
    let this_combat_damage_instances_taken = state.this_combat_damage_instances_taken;
    let energy_current = state.energy.current;
    let entangled = modifier_has(
        &state.entities[state.id_character].modifiers,
        InternalModifierKind::Entangled,
    );
    let card = |id_card: usize| {
        build_view_card_template(
            &state.entities[id_card],
            &state.id_pile_draw,
            this_turn_discards,
            this_combat_damage_instances_taken,
            energy_current,
            entangled,
        )
    };
    let relic = |id_relic: usize| build_view_relic(&state.entities[id_relic]);
    GameState {
        character: build_view_character(state),
        monsters: build_view_monsters(state),
        deck: state.id_deck.iter().copied().map(card).collect(),
        hand: state.id_hand.iter().copied().map(card).collect(),
        pile_draw: state.id_pile_draw.iter().copied().map(card).collect(),
        pile_discard: state.id_pile_discard.iter().copied().map(card).collect(),
        pile_exhaust: state.id_pile_exhaust.iter().copied().map(card).collect(),
        card_rewards: state.id_card_rewards.iter().copied().map(card).collect(),
        relics: crate::relics::iter_owned_relics(&state.id_relics)
            .map(|(_name, id)| build_view_relic(&state.entities[id]))
            .collect(),
        relic_rewards: state.id_relic_rewards.iter().copied().map(relic).collect(),
        energy: Energy {
            current: state.energy.current,
            max: state.energy.max,
        },
        map: build_view_map(state),
        phase: state.phase.into(),
    }
}

fn build_view_relic(e: &Entity) -> Relic {
    Relic {
        name: e.relic_name.into(),
        tier: e.relic_tier.into(),
        counter: e.relic_counter,
        used_up: e.relic_used_up,
    }
}

fn build_view_character(state: &InternalGameState) -> Character {
    let character = &state.entities[state.id_character];
    Character {
        name: character.character_name.to_string(),
        health: character.vitals.health,
        health_max: character.vitals.health_max,
        block: character.vitals.block,
        modifiers: build_view_modifiers(&character.modifiers),
        character_reward_roll_offset: character.character_reward_roll_offset,
        gold: character.character_gold,
    }
}

fn build_view_monsters(state: &InternalGameState) -> Vec<Monster> {
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
                let (mut base_damage, mut instances, block, buff, debuff) = match mv.intent {
                    InternalIntent::Attack { damage, instances } => {
                        (Some(damage), Some(instances), false, false, false)
                    }
                    InternalIntent::AttackBlock { damage, instances } => {
                        (Some(damage), Some(instances), true, false, false)
                    }
                    InternalIntent::AttackBuff { damage, instances } => {
                        (Some(damage), Some(instances), false, true, false)
                    }
                    InternalIntent::AttackDebuff { damage, instances } => {
                        (Some(damage), Some(instances), false, false, true)
                    }
                    InternalIntent::Block => (None, None, true, false, false),
                    InternalIntent::BlockBuff => (None, None, true, true, false),
                    InternalIntent::Buff => (None, None, false, true, false),
                    InternalIntent::Debuff => (None, None, false, false, true),
                    InternalIntent::DebuffPowerful => (None, None, false, false, true),
                    InternalIntent::Escape => (None, None, false, false, false),
                    InternalIntent::Sleep => (None, None, false, false, false),
                    InternalIntent::Stunned => (None, None, false, false, false),
                    InternalIntent::Unknown => (None, None, false, false, false),
                };

                // Hexaghost Divider's per-hit damage is dynamic (HP/12 + 1).
                // Override the static placeholder in MOVE_DIVIDER.intent so
                // the telegraph reflects what the player will actually take
                if m.monster_name == InternalMonsterName::Hexaghost
                    && move_idx == hexaghost::IDX_MOVE_DIVIDER
                {
                    base_damage = Some(character.vitals.health / 12 + 1);
                    instances = Some(HEXAGHOST_DIVIDER_HITS);
                }

                let damage = base_damage.map(|d| {
                    let str_stacks =
                        if modifier_has(&m.modifiers, InternalModifierKind::Strength) {
                            modifier_stacks(&m.modifiers, InternalModifierKind::Strength)
                        } else {
                            0
                        };
                    let mut scaled = scale_attack_damage(
                        d,
                        str_stacks,
                        modifier_has(&m.modifiers, InternalModifierKind::Weak),
                        modifier_has(mods_char, InternalModifierKind::Vulnerable),
                    );
                    if modifier_has(mods_char, InternalModifierKind::Intangible) && scaled > 1 {
                        scaled = 1;
                    }
                    scaled
                });

                Intent {
                    kind: mv.intent.into(),
                    damage,
                    instances,
                    block,
                    buff,
                    debuff,
                }
            } else {
                Intent {
                    kind: IntentKind::Unknown,
                    damage: None,
                    instances: None,
                    block: false,
                    buff: false,
                    debuff: false,
                }
            };

            Monster {
                name: m.monster_name.as_str().to_string(),
                monster_name: m.monster_name.into(),
                health: m.vitals.health,
                health_max: m.vitals.health_max,
                block: m.vitals.block,
                modifiers: build_view_modifiers(&m.modifiers),
                intent,
            }
        })
        .collect()
}

fn build_view_modifiers(mods: &Modifiers) -> Vec<Modifier> {
    let mut out = Vec::new();
    let mut bits = mods.active;
    while bits != 0 {
        let idx = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        let kind = InternalModifierKind::from_u8(idx as u8);
        out.push(Modifier {
            kind: kind.into(),
            stacks: mods.stacks[idx],
        });
    }
    out
}

fn build_view_card_template(
    card: &Entity,
    id_pile_draw: &[usize],
    this_turn_discards: u8,
    this_combat_damage_instances_taken: u8,
    energy_current: u8,
    entangled: bool,
) -> Card {
    let restriction_ok = is_play_restriction_satisfied(card.card_play_restriction, id_pile_draw);
    let entangled_blocks = entangled && card.card_kind == InternalCardKind::Attack;
    Card {
        name: if card.card_upgraded {
            format!("{}+", card.card_name.as_str())
        } else {
            card.card_name.as_str().to_string()
        },
        card_name: card.card_name.into(),
        kind: card.card_kind.into(),
        color: card.card_color.into(),
        rarity: card.card_rarity.into(),
        cost: card_effective_cost(
            card,
            this_turn_discards,
            this_combat_damage_instances_taken,
            energy_current,
        ),
        base_cost: card.card_cost,
        cost_kind: card.card_cost_kind.into(),
        upgraded: card.card_upgraded,
        exhaust: card.card_exhaust,
        ethereal: card.card_ethereal,
        innate: card.card_innate,
        requires_target: card.card_requires_target,
        retain: card.card_retain,
        free_to_play_once: card.card_free_to_play_once,
        playable: restriction_ok && !entangled_blocks,
        effects: card.card_effects[..card.card_effects_len as usize]
            .iter()
            .map(Effect::from_internal)
            .collect(),
    }
}

fn build_view_map(state: &InternalGameState) -> Map {
    let rooms = state
        .id_rooms
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    cell.map(|id_room| {
                        let room = &state.entities[id_room];
                        MapNode {
                            room_kind: room.room_kind.into(),
                            edges: edge_indices(room.edges).collect(),
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
    Map {
        rooms,
        y_current,
        x_current,
        boss_name: state.encounter_boss.as_str().to_string(),
    }
}
