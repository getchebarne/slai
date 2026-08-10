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
use super::macros::flat_variants;
use super::monster::PyMonster;
use super::monster::snapshot_monsters;
use super::potion::PyPotion;
use super::potion::snapshot_potion;
use super::relic::PyRelic;
use super::relic::snapshot_relic;

flat_variants!(PyMode {
    Map => PyModeMap as "ModeMap",
    RestSite => PyModeRestSite as "ModeRestSite",
    Chest => PyModeChest as "ModeChest",
    ChestOpened => PyModeChestOpened as "ModeChestOpened",
    CombatEnded => PyModeCombatEnded as "ModeCombatEnded",
    Combat => PyModeCombat as "ModeCombat" { hand: Vec<PyCard>, pile_draw: Vec<PyCard>, pile_discard: Vec<PyCard>, pile_exhaust: Vec<PyCard>, pile_stasis: Vec<PyCard>, energy: PyEnergy, monsters: Vec<PyMonster>, discover: Vec<PyCard>, bomb_countdown: u8 },
    Reward => PyModeReward as "ModeReward" { cards: Vec<Vec<PyCard>>, relics: Vec<PyRelic>, potions: Vec<PyPotion>, gold: Option<u16> },
    Shop => PyModeShop as "ModeShop" { cards: Vec<PyCard>, card_prices: Vec<u16>, relics: Vec<PyRelic>, relic_prices: Vec<u16>, potions: Vec<PyPotion>, potion_prices: Vec<u16>, purge_cost: u16 },
    Event => PyModeEvent as "ModeEvent" { kind: PyEventKind, options: Vec<Vec<PyEffect>>, consumed: bool },
});

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Energy",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEnergy {
    pub energy_current: u8,
    pub energy_max: u8,
}

pub(crate) fn snapshot_mode(state: &GameState, mode: &Mode) -> PyMode {
    match mode {
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
            id_stasis_cards,
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
            pile_stasis: id_stasis_cards
                .iter()
                .flatten()
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
            reward_id_relics,
            reward_id_potions,
            reward_gold,
            ..
        } => PyMode::Reward(PyModeReward {
            cards: reward_id_cards
                .iter()
                .map(|bundle| bundle.iter().map(|&id| snapshot_card(state, id)).collect())
                .collect(),
            relics: reward_id_relics
                .iter()
                .map(|&id| snapshot_relic(&state.entities[id]))
                .collect(),
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
                    let option = &state.entities[id];
                    option.event_option_effects[..option.event_option_effects_len as usize]
                        .iter()
                        .map(snapshot_effect)
                        .collect()
                })
                .collect(),
            consumed: *consumed,
        }),
    }
}
