use pyo3::prelude::*;

use crate::game::GameState;
use crate::types::ChestKind;

use super::card::PyCard;
use super::card::snapshot_card;
use super::effect::PyEffect;
use super::effect::snapshot_effect;
use super::event::PyEventName;
use super::macros::mirror_enum;
use super::monster::PyMonster;
use super::monster::snapshot_monsters;
use super::potion::PyPotion;
use super::potion::snapshot_potion;
use super::relic::PyRelic;
use super::relic::snapshot_relic;

mirror_enum!(PyChestKind from ChestKind, "ChestKind", skip_from_py_object, {
    Small, Medium, Large,
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

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Combat",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCombat {
    pub hand: Vec<PyCard>,
    pub pile_draw: Vec<PyCard>,
    pub pile_discard: Vec<PyCard>,
    pub pile_exhaust: Vec<PyCard>,
    pub pile_stasis: Vec<PyCard>,
    pub energy: PyEnergy,
    pub monsters: Vec<PyMonster>,
    pub discover: Vec<PyCard>,
    pub bomb_countdown: u8,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Reward",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyReward {
    pub cards: Vec<Vec<PyCard>>,
    pub relics: Vec<PyRelic>,
    pub potions: Vec<PyPotion>,
    pub gold: Option<u16>,
    // Boss rewards roll mutually exclusive relics: taking one discards the rest
    pub relics_exclusive: bool,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Shop",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyShop {
    pub cards: Vec<PyCard>,
    pub card_prices: Vec<u16>,
    pub relics: Vec<PyRelic>,
    pub relic_prices: Vec<u16>,
    pub potions: Vec<PyPotion>,
    pub potion_prices: Vec<u16>,
    pub purge_cost: u16,
    pub purged: bool,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Event",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyEvent {
    pub name: PyEventName,
    // Per-event control state: stage / attempts / searches, meaning per event
    pub stage: u8,
    // Dead Adventurer's without-replacement loot draws
    pub found_gold: bool,
    pub found_nothing: bool,
    pub found_relic: bool,
    pub options: Vec<Vec<PyEffect>>,
    pub consumed: bool,

    // Entities the spawn rolled/staked; option targets resolve against these via
    // the CandidatePoolEventRoll* pools
    pub roll_cards: Vec<PyCard>,
    pub roll_relics: Vec<PyRelic>,
    pub roll_potions: Vec<PyPotion>,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "RestSite",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyRestSite {
    pub done: bool,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Chest",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyChest {
    pub kind: PyChestKind,
    pub opened: bool,
}

pub(crate) fn snapshot_combat(state: &GameState) -> PyCombat {
    let combat = &state.combat;
    PyCombat {
        hand: combat
            .id_card_hand
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        pile_draw: combat
            .id_card_draw
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        pile_discard: combat
            .id_card_discard
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        pile_exhaust: combat
            .id_card_exhaust
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        pile_stasis: combat
            .id_card_stasis
            .iter()
            .flatten()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        energy: PyEnergy {
            energy_current: combat.energy.energy_current,
            energy_max: combat.energy.energy_max,
        },
        monsters: snapshot_monsters(state),
        discover: combat
            .id_card_discover
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        bomb_countdown: combat.bomb_countdown,
    }
}

pub(crate) fn snapshot_reward(state: &GameState) -> PyReward {
    let reward = &state.reward;
    PyReward {
        cards: reward
            .id_cards
            .iter()
            .map(|bundle| bundle.iter().map(|&id| snapshot_card(state, id)).collect())
            .collect(),
        relics: reward
            .id_relics
            .iter()
            .map(|&id| snapshot_relic(id, &state.entities[id]))
            .collect(),
        potions: reward
            .id_potions
            .iter()
            .map(|&id| snapshot_potion(id, &state.entities[id]))
            .collect(),
        gold: reward.gold,
        relics_exclusive: reward.relics_exclusive,
    }
}

pub(crate) fn snapshot_shop(state: &GameState) -> PyShop {
    let shop = &state.shop;
    PyShop {
        cards: shop
            .cards
            .iter()
            .map(|&(id, _)| snapshot_card(state, id))
            .collect(),
        card_prices: shop.cards.iter().map(|&(_, price)| price).collect(),
        relics: shop
            .relics
            .iter()
            .map(|&(id, _)| snapshot_relic(id, &state.entities[id]))
            .collect(),
        relic_prices: shop.relics.iter().map(|&(_, price)| price).collect(),
        potions: shop
            .potions
            .iter()
            .map(|&(id, _)| snapshot_potion(id, &state.entities[id]))
            .collect(),
        potion_prices: shop.potions.iter().map(|&(_, price)| price).collect(),
        purge_cost: shop.purge_cost,
        purged: shop.purged,
    }
}

pub(crate) fn snapshot_event(state: &GameState) -> PyEvent {
    let event = &state.event;
    PyEvent {
        name: event.name.into(),
        stage: event.stage,
        found_gold: event.found_gold,
        found_nothing: event.found_nothing,
        found_relic: event.found_relic,
        options: event
            .id_event_options
            .iter()
            .map(|&id| {
                let option = &state.entities[id];
                option.event_option_effects[..option.event_option_effects_len as usize]
                    .iter()
                    .map(snapshot_effect)
                    .collect()
            })
            .collect(),
        consumed: event.consumed,
        roll_cards: event
            .id_roll_card
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        roll_relics: event
            .id_roll_relic
            .iter()
            .map(|&id| snapshot_relic(id, &state.entities[id]))
            .collect(),
        roll_potions: event
            .id_roll_potion
            .iter()
            .map(|&id| snapshot_potion(id, &state.entities[id]))
            .collect(),
    }
}

pub(crate) fn snapshot_rest_site(state: &GameState) -> PyRestSite {
    PyRestSite {
        done: state.rest_site.consumed,
    }
}

pub(crate) fn snapshot_chest(state: &GameState) -> PyChest {
    PyChest {
        kind: state.chest.chest_kind.into(),
        opened: state.chest.chest_opened,
    }
}
