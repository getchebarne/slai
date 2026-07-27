use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::game::GameState;
use crate::types::Mode;

use super::card::PyCard;
use super::card::snapshot_card;
use super::effect::PyEffect;
use super::effect::snapshot_effect;
use super::event::PyEventKind;
use super::event::snapshot_event_kind;
use super::macros::variant_union;
use super::monster::PyMonster;
use super::monster::snapshot_monsters;
use super::potion::PyPotion;
use super::potion::snapshot_potion;
use super::relic::PyRelic;
use super::relic::snapshot_relic;

#[pyclass(skip_from_py_object, frozen, name = "ModeMap", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeMap;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "ModeRestSite",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyModeRestSite;

#[pyclass(skip_from_py_object, frozen, name = "ModeChest", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyModeChest;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "ModeChestOpened",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyModeChestOpened;

#[pyclass(
    skip_from_py_object,
    frozen,
    name = "ModeCombatEnded",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyModeCombatEnded;

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "ModeCombat",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyModeCombat {
    pub hand: Vec<PyCard>,
    pub pile_draw: Vec<PyCard>,
    pub pile_discard: Vec<PyCard>,
    pub pile_exhaust: Vec<PyCard>,
    pub energy: PyEnergy,
    pub monsters: Vec<PyMonster>,
    pub discover: Vec<PyCard>,
    pub bomb_countdown: u8,
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "ModeReward",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyModeReward {
    pub cards: Vec<PyCard>,
    pub relic: Option<PyRelic>,
    pub potions: Vec<PyPotion>,
    pub gold: Option<u16>,
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "ModeShop",
    module = "slai.slai"
)]
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

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "ModeEvent",
    module = "slai.slai"
)]
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

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "Energy",
    module = "slai.slai"
)]
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

pub(crate) fn snapshot_mode(state: &GameState) -> PyMode {
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
            bomb_countdown,
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
            bomb_countdown: *bomb_countdown,
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
            shop_purge_cost,
        } => PyMode::Shop(PyModeShop {
            cards: shop_id_cards
                .iter()
                .map(|&id| snapshot_card(state, id))
                .collect(),
            card_prices: shop_id_cards
                .iter()
                .map(|&id| state.entities[id].price)
                .collect(),
            relics: shop_id_relics
                .iter()
                .map(|&id| snapshot_relic(&state.entities[id]))
                .collect(),
            relic_prices: shop_id_relics
                .iter()
                .map(|&id| state.entities[id].price)
                .collect(),
            potions: shop_id_potions
                .iter()
                .map(|&id| snapshot_potion(&state.entities[id]))
                .collect(),
            potion_prices: shop_id_potions
                .iter()
                .map(|&id| state.entities[id].price)
                .collect(),
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
