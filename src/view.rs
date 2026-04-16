// View layer: decoupled state snapshot for Python consumption.
//
// Naming convention: every `#[pyclass]` here is defined with a `View*` Rust
// name (to avoid collisions with engine types like `cards::Card`,
// `state::Character`, etc.) and exposed to Python under its unprefixed name
// via the `name = "..."` attribute.

use pyo3::prelude::*;

use crate::consts::{FACTOR_VULN, MAX_MONSTERS};
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent};
use crate::map::edge_indices;
use crate::modifier::{ModifierKind, modifier_has, modifier_stacks};
use crate::state::{GameState, Location};
use crate::types::Phase;
use crate::utils::fill_alive_monster_ids;

// ───────── Selection variants ─────────

#[pyclass(name = "SelectionAll", frozen)]
#[derive(Debug, Clone)]
pub struct ViewSelectionAll;

#[pymethods]
impl ViewSelectionAll {
    #[new]
    fn new() -> Self {
        Self
    }
}

#[pyclass(name = "SelectionRandom", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewSelectionRandom {
    pub count: u8,
}

#[pymethods]
impl ViewSelectionRandom {
    #[new]
    fn new(count: u8) -> Self {
        Self { count }
    }
}

#[pyclass(name = "SelectionInput", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewSelectionInput {
    pub count: u8,
}

#[pymethods]
impl ViewSelectionInput {
    #[new]
    fn new(count: u8) -> Self {
        Self { count }
    }
}

// ───────── Effect variants ─────────
//
// One pyclass per `EffectKind` variant that appears in static card/monster
// definitions. Each carries the variant's data fields plus `candidates`
// (stringified `CandidatePool`, typed as `CandidatePool` StrEnum in the stub)
// and `selection` (one of `SelectionAll` / `SelectionRandom` / `SelectionInput`).

#[pyclass(name = "DamagePhysical", frozen, get_all)]
#[derive(Debug)]
pub struct ViewDamagePhysical {
    pub base: u16,
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "BlockGain", frozen, get_all)]
#[derive(Debug)]
pub struct ViewBlockGain {
    pub amount: u16,
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "ModifierGain", frozen, get_all)]
#[derive(Debug)]
pub struct ViewModifierGain {
    pub kind: String,
    pub stacks: i16,
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "ModifierRemove", frozen, get_all)]
#[derive(Debug)]
pub struct ViewModifierRemove {
    pub kind: String,
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "EnergyGain", frozen, get_all)]
#[derive(Debug)]
pub struct ViewEnergyGain {
    pub amount: u8,
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "AddShivs", frozen, get_all)]
#[derive(Debug)]
pub struct ViewAddShivs {
    pub count: u8,
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "CardDraw", frozen, get_all)]
#[derive(Debug)]
pub struct ViewCardDraw {
    pub count: u8,
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "CardDiscard", frozen, get_all)]
#[derive(Debug)]
pub struct ViewCardDiscard {
    pub candidates: String,
    pub selection: PyObject,
}

#[pyclass(name = "CalculatedGamble", frozen, get_all)]
#[derive(Debug)]
pub struct ViewCalculatedGamble {
    pub candidates: String,
    pub selection: PyObject,
}

// ───────── Existing view types ─────────

#[pyclass(name = "Card", frozen)]
#[derive(Debug)]
pub struct ViewCard {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub kind: String,
    #[pyo3(get)]
    pub color: String,
    #[pyo3(get)]
    pub rarity: String,
    #[pyo3(get)]
    pub cost: u8,
    #[pyo3(get)]
    pub upgraded: bool,
    #[pyo3(get)]
    pub exhaust: bool,
    #[pyo3(get)]
    pub innate: bool,
    #[pyo3(get)]
    pub requires_target: bool,
    // No `#[pyo3(get)]`: `Vec<PyObject>` has no auto-generated getter;
    // expose it via a manual `#[getter]` below that clones references.
    pub effects: Vec<PyObject>,
}

#[pymethods]
impl ViewCard {
    #[getter]
    fn effects(&self, py: Python<'_>) -> Vec<PyObject> {
        self.effects.iter().map(|o| o.clone_ref(py)).collect()
    }
}

#[pyclass(name = "Modifier", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewModifier {
    pub kind: String,
    pub stacks: i16,
}

#[pyclass(name = "Character", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewCharacter {
    pub name: String,
    pub health_current: u16,
    pub health_max: u16,
    pub block_current: u16,
    pub modifiers: Vec<ViewModifier>,
    pub card_reward_roll_offset: i8,
}

#[pyclass(name = "Intent", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewIntent {
    pub damage: Option<u16>,
    pub instances: Option<u8>,
    pub block: bool,
    pub buff: bool,
    pub debuff: bool,
}

#[pyclass(name = "Monster", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewMonster {
    pub name: String,
    pub health_current: u16,
    pub health_max: u16,
    pub block_current: u16,
    pub modifiers: Vec<ViewModifier>,
    pub intent: ViewIntent,
}

#[pyclass(name = "Energy", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewEnergy {
    pub current: u8,
    pub max: u8,
}

#[pyclass(name = "Room", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewMapNode {
    pub room_type: String,
    pub edges: Vec<usize>,
}

#[pyclass(name = "Map", frozen, get_all)]
#[derive(Debug, Clone)]
pub struct ViewMap {
    pub rooms: Vec<Vec<Option<ViewMapNode>>>,
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
}

#[pyclass(name = "GameState", frozen)]
#[derive(Debug)]
pub struct ViewGameState {
    #[pyo3(get)]
    pub character: ViewCharacter,
    #[pyo3(get)]
    pub monsters: Vec<ViewMonster>,
    // Card piles are stored as `Vec<Py<ViewCard>>` so getters can clone
    // refs without requiring `ViewCard: Clone` (which is impossible because
    // each card contains a `Vec<PyObject>` of effect variants).
    pub deck: Vec<Py<ViewCard>>,
    pub hand: Vec<Py<ViewCard>>,
    pub pile_draw: Vec<Py<ViewCard>>,
    pub pile_disc: Vec<Py<ViewCard>>,
    pub pile_exhaust: Vec<Py<ViewCard>>,
    pub reward_combat: Vec<Py<ViewCard>>,
    #[pyo3(get)]
    pub energy: ViewEnergy,
    #[pyo3(get)]
    pub map: ViewMap,
    #[pyo3(get)]
    pub phase: String,
    #[pyo3(get)]
    pub discard_count: Option<u8>,
}

#[pymethods]
impl ViewGameState {
    #[getter]
    fn deck(&self, py: Python<'_>) -> Vec<Py<ViewCard>> {
        self.deck.iter().map(|c| c.clone_ref(py)).collect()
    }
    #[getter]
    fn hand(&self, py: Python<'_>) -> Vec<Py<ViewCard>> {
        self.hand.iter().map(|c| c.clone_ref(py)).collect()
    }
    #[getter]
    fn pile_draw(&self, py: Python<'_>) -> Vec<Py<ViewCard>> {
        self.pile_draw.iter().map(|c| c.clone_ref(py)).collect()
    }
    #[getter]
    fn pile_disc(&self, py: Python<'_>) -> Vec<Py<ViewCard>> {
        self.pile_disc.iter().map(|c| c.clone_ref(py)).collect()
    }
    #[getter]
    fn pile_exhaust(&self, py: Python<'_>) -> Vec<Py<ViewCard>> {
        self.pile_exhaust.iter().map(|c| c.clone_ref(py)).collect()
    }
    #[getter]
    fn reward_combat(&self, py: Python<'_>) -> Vec<Py<ViewCard>> {
        self.reward_combat.iter().map(|c| c.clone_ref(py)).collect()
    }
}

// ───────── Build functions ─────────

pub fn build_view(py: Python<'_>, state: &GameState) -> ViewGameState {
    let wrap = |card: ViewCard| Py::new(py, card).unwrap();
    ViewGameState {
        character: build_view_character(state),
        monsters: build_view_monsters(state),
        deck: state
            .id_deck
            .iter()
            .map(|&id_card| wrap(build_view_card_template(py, &state.entities[id_card])))
            .collect(),
        hand: state
            .id_hand
            .iter()
            .map(|&id_card| wrap(build_view_card_template(py, &state.entities[id_card])))
            .collect(),
        pile_draw: build_view_pile(py, &state.entities, &state.id_draw_pile),
        pile_disc: build_view_pile(py, &state.entities, &state.id_discard_pile),
        pile_exhaust: build_view_pile(py, &state.entities, &state.id_exhaust_pile),
        reward_combat: state
            .id_card_rewards
            .iter()
            .map(|&id_card| wrap(build_view_card_template(py, &state.entities[id_card])))
            .collect(),
        energy: ViewEnergy {
            current: state.energy.current,
            max: state.energy.max,
        },
        map: build_view_map(state),
        phase: phase_variant_name(state.phase),
        discard_count: match state.phase {
            Phase::CombatAwaitDiscard { num } => Some(num),
            _ => None,
        },
    }
}

pub fn phase_variant_name(phase: Phase) -> String {
    match phase {
        Phase::Map => "Map".to_string(),
        Phase::CombatDefault => "CombatDefault".to_string(),
        Phase::CombatAwaitDiscard { .. } => "CombatAwaitDiscard".to_string(),
        Phase::CombatReward => "CombatReward".to_string(),
        Phase::RestSite => "RestSite".to_string(),
        Phase::GameOver => "GameOver".to_string(),
    }
}

fn build_view_pile(py: Python<'_>, entities: &[Entity], pile: &[usize]) -> Vec<Py<ViewCard>> {
    pile.iter()
        .map(|&id| Py::new(py, build_view_card_template(py, &entities[id])).unwrap())
        .collect()
}

fn build_view_character(state: &GameState) -> ViewCharacter {
    let character = &state.entities[state.id_character];
    ViewCharacter {
        name: character.character_name.to_string(),
        health_current: character.vitals.health,
        health_max: character.vitals.health_max,
        block_current: character.vitals.block,
        modifiers: build_view_modifiers(&character.modifiers),
        card_reward_roll_offset: character.reward_roll_offset,
    }
}

fn build_view_monsters(state: &GameState) -> Vec<ViewMonster> {
    let character_modifiers = &state.entities[state.id_character].modifiers;

    // Stack locals
    let mut buf_alive = [0usize; MAX_MONSTERS];

    let n = fill_alive_monster_ids(state, &mut buf_alive);
    buf_alive[..n]
        .iter()
        .map(|&id_monster| {
            let m = &state.entities[id_monster];

            let intent = if let Some(move_idx) = m.move_current {
                let mv = &m.moves[move_idx];
                let (base_damage, instances, block, buff, debuff) = match mv.intent {
                    Intent::Attack { damage, instances } => {
                        (Some(damage), Some(instances), false, false, false)
                    }
                    Intent::AttackBlock { damage, instances } => {
                        (Some(damage), Some(instances), true, false, false)
                    }
                    Intent::AttackBuff { damage, instances } => {
                        (Some(damage), Some(instances), false, true, false)
                    }
                    Intent::Block => (None, None, true, false, false),
                    Intent::BlockBuff => (None, None, true, true, false),
                    Intent::Buff => (None, None, false, true, false),
                    Intent::Debuff => (None, None, false, false, true),
                    Intent::DebuffPowerful => (None, None, false, false, true),
                };

                let damage = base_damage.map(|d| {
                    let mut dmg = d as f32;
                    if modifier_has(&m.modifiers, ModifierKind::Strength) {
                        dmg += modifier_stacks(&m.modifiers, ModifierKind::Strength) as f32;
                    }
                    if modifier_has(&m.modifiers, ModifierKind::Weak) {
                        dmg *= 0.75;
                    }
                    if modifier_has(character_modifiers, ModifierKind::Vulnerable) {
                        dmg *= FACTOR_VULN;
                    }
                    dmg as u16
                });

                ViewIntent {
                    damage,
                    instances,
                    block,
                    buff,
                    debuff,
                }
            } else {
                ViewIntent {
                    damage: None,
                    instances: None,
                    block: false,
                    buff: false,
                    debuff: false,
                }
            };

            ViewMonster {
                name: m.monster_name.as_str().to_string(),
                health_current: m.vitals.health,
                health_max: m.vitals.health_max,
                block_current: m.vitals.block,
                modifiers: build_view_modifiers(&m.modifiers),
                intent,
            }
        })
        .collect()
}

fn build_view_modifiers(mods: &crate::modifier::Modifiers) -> Vec<ViewModifier> {
    let mut out = Vec::new();
    let mut bits = mods.active;
    while bits != 0 {
        let idx = bits.trailing_zeros() as usize;
        bits &= bits - 1;
        let kind = ModifierKind::from_u8(idx as u8);
        out.push(ViewModifier {
            kind: format!("{:?}", kind),
            stacks: mods.stacks[idx],
        });
    }
    out
}

fn build_view_card_template(py: Python<'_>, card: &Entity) -> ViewCard {
    ViewCard {
        name: if card.card_upgraded {
            format!("{}+", card.card_name.as_str())
        } else {
            card.card_name.as_str().to_string()
        },
        kind: format!("{:?}", card.card_kind),
        color: format!("{:?}", card.card_color),
        rarity: format!("{:?}", card.card_rarity),
        cost: card.card_cost,
        upgraded: card.card_upgraded,
        exhaust: card.card_exhaust,
        innate: card.card_innate,
        requires_target: card.card_requires_target,
        effects: card
            .card_effects
            .iter()
            .map(|e| view_effect(py, e))
            .collect(),
    }
}

/// Construct the right per-variant pyclass for a card/monster effect.
///
/// Static card and monster definitions always use `Target::Resolve` — the
/// `Direct` case is engine-internal (produced by the resolver after fan-out)
/// and never reaches the view layer. An unexpected `Direct` or an unlisted
/// `EffectKind` variant is a developer error: panic rather than hide it.
fn view_effect(py: Python<'_>, effect: &Effect) -> PyObject {
    let (candidates_str, selection_obj) = match effect.target {
        Target::Resolve {
            candidates,
            selection,
        } => {
            let cand = candidate_pool_str(candidates);
            let sel = match selection {
                SelectionKind::All => Py::new(py, ViewSelectionAll).unwrap().into_py(py),
                SelectionKind::Random { count } => Py::new(py, ViewSelectionRandom { count })
                    .unwrap()
                    .into_py(py),
                SelectionKind::Input { count } => Py::new(py, ViewSelectionInput { count })
                    .unwrap()
                    .into_py(py),
            };
            (cand, sel)
        }
        Target::Direct(None) => {
            let sel = Py::new(py, ViewSelectionAll).unwrap().into_py(py);
            ("None".to_string(), sel)
        }
        Target::Direct(Some(_)) => panic!(
            "view_effect: unexpected Direct(Some) on static card effect: {:?}",
            effect,
        ),
    };

    match effect.kind {
        EffectKind::DamagePhysical { base } => Py::new(
            py,
            ViewDamagePhysical {
                base,
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::BlockGain { amount } => Py::new(
            py,
            ViewBlockGain {
                amount,
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::ModifierGain { kind, stacks } => Py::new(
            py,
            ViewModifierGain {
                kind: format!("{:?}", kind),
                stacks,
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::ModifierRemove { kind } => Py::new(
            py,
            ViewModifierRemove {
                kind: format!("{:?}", kind),
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::EnergyGain { amount } => Py::new(
            py,
            ViewEnergyGain {
                amount,
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::AddShivs { count } => Py::new(
            py,
            ViewAddShivs {
                count,
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::CardDraw { count } => Py::new(
            py,
            ViewCardDraw {
                count,
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::CardDiscard => Py::new(
            py,
            ViewCardDiscard {
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        EffectKind::CalculatedGamble => Py::new(
            py,
            ViewCalculatedGamble {
                candidates: candidates_str,
                selection: selection_obj,
            },
        )
        .unwrap()
        .into_py(py),
        other => unreachable!(
            "view_effect: unexpected EffectKind variant on static card effect: {:?}",
            other
        ),
    }
}

fn candidate_pool_str(c: CandidatePool) -> String {
    format!("{:?}", c)
}

fn build_view_map(state: &GameState) -> ViewMap {
    let rooms = state
        .id_rooms
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    cell.map(|id_room| {
                        let room = &state.entities[id_room];
                        ViewMapNode {
                            room_type: format!("{:?}", room.room_type),
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
        Location::BossRoom => (Some(crate::consts::MAP_HEIGHT), Some(0)),
    };
    ViewMap {
        rooms,
        y_current,
        x_current,
    }
}
