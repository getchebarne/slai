// FFI boundary: every #[pyclass] type lives here. Internal engine modules
// must not import pyo3
//
// Naming: structs that snapshot internal engine state (GameState, Card, ...)
// take the bare name. Where the bare name would collide with an internal type
// at the Rust level (engine `state::GameState`, `entity::Intent`), we alias
// the internal import below

use pyo3::prelude::*;

use crate::action::Action as InternalAction;
use crate::consts::{FACTOR_VULN, MAP_HEIGHT, MAX_MONSTERS};
use crate::effect::{
    CandidatePool as InternalCandidatePool, Effect as InternalEffect, EffectKind, SelectionKind,
    Target as InternalTarget,
};
use crate::entity::{
    CardCostKind as InternalCardCostKind, Entity, Intent as InternalIntent, card_effective_cost,
    is_play_restriction_satisfied,
};
use crate::map::edge_indices;
use crate::modifier::{
    ModifierKind as InternalModifierKind, Modifiers, modifier_has, modifier_stacks,
};
use crate::state::{GameState as InternalGameState, Location};
use crate::types::{
    CardColor as InternalCardColor, CardKind as InternalCardKind, CardRarity as InternalCardRarity,
    Phase as InternalPhase, RoomKind as InternalRoomKind,
};
use crate::utils::fill_alive_monster_ids;

// ───────── Unit enum mirrors ─────────

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

#[pyclass(eq, eq_int, hash, frozen, name = "ModifierKind")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierKind {
    Accuracy,
    AfterImage,
    Angry,
    Artifact,
    Blur,
    Burst,
    Choke,
    CorpseExplosion,
    CurlUp,
    Dexterity,
    DoubleDamage,
    DrawCardNextTurn,
    Envenom,
    Frail,
    InfiniteBlades,
    Intangible,
    ModeShift,
    NextTurnBlock,
    NextTurnEnergy,
    NoDraw,
    NoxiousFumes,
    Phantasmal,
    Poison,
    Retain,
    Ritual,
    Shackled,
    SharpHide,
    Splittable,
    SporeCloud,
    Strength,
    Thorns,
    ThousandCuts,
    ToolsOfTheTrade,
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
            InternalModifierKind::Blur => Self::Blur,
            InternalModifierKind::Burst => Self::Burst,
            InternalModifierKind::Choke => Self::Choke,
            InternalModifierKind::CorpseExplosion => Self::CorpseExplosion,
            InternalModifierKind::CurlUp => Self::CurlUp,
            InternalModifierKind::Dexterity => Self::Dexterity,
            InternalModifierKind::DoubleDamage => Self::DoubleDamage,
            InternalModifierKind::DrawCardNextTurn => Self::DrawCardNextTurn,
            InternalModifierKind::Envenom => Self::Envenom,
            InternalModifierKind::Frail => Self::Frail,
            InternalModifierKind::InfiniteBlades => Self::InfiniteBlades,
            InternalModifierKind::Intangible => Self::Intangible,
            InternalModifierKind::ModeShift => Self::ModeShift,
            InternalModifierKind::NextTurnBlock => Self::NextTurnBlock,
            InternalModifierKind::NextTurnEnergy => Self::NextTurnEnergy,
            InternalModifierKind::NoDraw => Self::NoDraw,
            InternalModifierKind::NoxiousFumes => Self::NoxiousFumes,
            InternalModifierKind::Phantasmal => Self::Phantasmal,
            InternalModifierKind::Poison => Self::Poison,
            InternalModifierKind::Retain => Self::Retain,
            InternalModifierKind::Ritual => Self::Ritual,
            InternalModifierKind::Shackled => Self::Shackled,
            InternalModifierKind::SharpHide => Self::SharpHide,
            InternalModifierKind::Splittable => Self::Splittable,
            InternalModifierKind::SporeCloud => Self::SporeCloud,
            InternalModifierKind::Strength => Self::Strength,
            InternalModifierKind::Thorns => Self::Thorns,
            InternalModifierKind::ThousandCuts => Self::ThousandCuts,
            InternalModifierKind::ToolsOfTheTrade => Self::ToolsOfTheTrade,
            InternalModifierKind::Vulnerable => Self::Vulnerable,
            InternalModifierKind::Weak => Self::Weak,
            InternalModifierKind::WraithForm => Self::WraithForm,
            InternalModifierKind::Count => {
                unreachable!("ModifierKind::Count is a sentinel, never a real modifier")
            }
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

// ───────── Complex enum mirrors ─────────

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
    Random { count: u8 },
    Input { count: u8 },
}

impl From<SelectionKind> for Selection {
    fn from(s: SelectionKind) -> Self {
        match s {
            SelectionKind::All => Self::All {},
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

#[pyclass(eq, hash, frozen, name = "Action")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    CardPlay {
        idx_hand: usize,
        idx_monster: Option<usize>,
    },
    EndTurn {},
    CardDiscard {
        indices_hand: Vec<usize>,
    },
    CardRetain {
        indices_hand: Vec<usize>,
    },
    CardSetup {
        idx_hand: usize,
    },
    CardNightmare {
        idx_hand: usize,
    },
    RoomSelect {
        idx_column: usize,
    },
    CardRewardSelect {
        idx_reward: usize,
    },
    CardRewardSkip {},
    RestSiteRest {},
    RestSiteCardUpgrade {
        idx_deck: usize,
    },
}

impl From<Action> for InternalAction {
    fn from(a: Action) -> Self {
        match a {
            Action::CardPlay {
                idx_hand,
                idx_monster,
            } => InternalAction::CardPlay {
                idx_hand,
                idx_monster,
            },
            Action::EndTurn {} => InternalAction::EndTurn,
            Action::CardDiscard { indices_hand } => InternalAction::CardDiscard { indices_hand },
            Action::CardRetain { indices_hand } => InternalAction::CardRetain { indices_hand },
            Action::CardSetup { idx_hand } => InternalAction::CardSetup { idx_hand },
            Action::CardNightmare { idx_hand } => InternalAction::CardNightmare { idx_hand },
            Action::RoomSelect { idx_column } => InternalAction::RoomSelect { idx_column },
            Action::CardRewardSelect { idx_reward } => {
                InternalAction::CardRewardSelect { idx_reward }
            }
            Action::CardRewardSkip {} => InternalAction::CardRewardSkip,
            Action::RestSiteRest {} => InternalAction::RestSiteRest,
            Action::RestSiteCardUpgrade { idx_deck } => {
                InternalAction::RestSiteCardUpgrade { idx_deck }
            }
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
    EndlessAgonyAddCopy {
        upgraded: bool,
        target: Option<Target>,
    },
    BulletTimeProc {
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
    ShivAdd {
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
            EffectKind::EndlessAgonyAddCopy { upgraded } => {
                Self::EndlessAgonyAddCopy { upgraded, target }
            }
            EffectKind::BulletTimeProc => Self::BulletTimeProc { target },
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
            EffectKind::ShivAdd { count, upgraded } => Self::ShivAdd {
                count,
                upgraded,
                target,
            },
            EffectKind::CardDraw { count } => Self::CardDraw { count, target },
            EffectKind::DrawUpTo { target: n } => Self::DrawUpTo {
                target: n,
                target_field: target,
            },
            EffectKind::CardDiscard => Self::CardDiscard { target },
            EffectKind::CalculatedGamble => Self::CalculatedGamble { target },
            other => unreachable!(
                "Effect::from_internal: unexpected EffectKind on static card effect: {:?}",
                other
            ),
        }
    }
}

// ───────── View structs ─────────

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
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

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<Modifier>,
    pub reward_roll_offset: i8,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Intent {
    pub damage: Option<u16>,
    pub instances: Option<u8>,
    pub block: bool,
    pub buff: bool,
    pub debuff: bool,
}

#[pyclass(frozen, get_all)]
#[derive(Debug, Clone)]
pub struct Monster {
    pub name: String,
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
    pub energy: Energy,
    pub map: Map,
    pub phase: Phase,
}

// ───────── Build functions ─────────

pub fn build_view(state: &InternalGameState) -> GameState {
    let this_turn_discards = state.this_turn_discards;
    let this_combat_damage_instances_taken = state.this_combat_damage_instances_taken;
    let energy_current = state.energy.current;
    let card = |id_card: usize| {
        build_view_card_template(
            &state.entities[id_card],
            &state.id_pile_draw,
            this_turn_discards,
            this_combat_damage_instances_taken,
            energy_current,
        )
    };
    GameState {
        character: build_view_character(state),
        monsters: build_view_monsters(state),
        deck: state.id_deck.iter().copied().map(card).collect(),
        hand: state.id_hand.iter().copied().map(card).collect(),
        pile_draw: state.id_pile_draw.iter().copied().map(card).collect(),
        pile_discard: state.id_pile_discard.iter().copied().map(card).collect(),
        pile_exhaust: state.id_pile_exhaust.iter().copied().map(card).collect(),
        card_rewards: state.id_card_rewards.iter().copied().map(card).collect(),
        energy: Energy {
            current: state.energy.current,
            max: state.energy.max,
        },
        map: build_view_map(state),
        phase: state.phase.into(),
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
        reward_roll_offset: character.reward_roll_offset,
    }
}

fn build_view_monsters(state: &InternalGameState) -> Vec<Monster> {
    let mods_char = &state.entities[state.id_character].modifiers;
    let mut buf_alive = [0usize; MAX_MONSTERS];
    let n = fill_alive_monster_ids(state, &mut buf_alive);
    buf_alive[..n]
        .iter()
        .map(|&id_monster| {
            let m = &state.entities[id_monster];

            let intent = if let Some(move_idx) = m.move_current {
                let mv = &m.moves[move_idx];
                let (base_damage, instances, block, buff, debuff) = match mv.intent {
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
                    InternalIntent::Unknown => (None, None, false, false, false),
                };

                let damage = base_damage.map(|d| {
                    let mut dmg = d as f32;
                    if modifier_has(&m.modifiers, InternalModifierKind::Strength) {
                        dmg += modifier_stacks(&m.modifiers, InternalModifierKind::Strength) as f32;
                    }
                    if modifier_has(&m.modifiers, InternalModifierKind::Weak) {
                        dmg *= 0.75;
                    }
                    if modifier_has(mods_char, InternalModifierKind::Vulnerable) {
                        dmg *= FACTOR_VULN;
                    }
                    dmg as u16
                });

                Intent {
                    damage,
                    instances,
                    block,
                    buff,
                    debuff,
                }
            } else {
                Intent {
                    damage: None,
                    instances: None,
                    block: false,
                    buff: false,
                    debuff: false,
                }
            };

            Monster {
                name: m.monster_name.as_str().to_string(),
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
) -> Card {
    Card {
        name: if card.card_upgraded {
            format!("{}+", card.card_name.as_str())
        } else {
            card.card_name.as_str().to_string()
        },
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
        playable: is_play_restriction_satisfied(card.card_play_restriction, id_pile_draw),
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
    }
}
