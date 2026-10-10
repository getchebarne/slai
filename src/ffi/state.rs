use pyo3::prelude::*;

use crate::game::GameState;

use super::card::PyCard;
use super::card::snapshot_card;
use super::character::PyCharacter;
use super::character::snapshot_character;
use super::context::PyChest;
use super::context::PyCombat;
use super::context::PyEvent;
use super::context::PyRestSite;
use super::context::PyReward;
use super::context::PyShop;
use super::context::snapshot_chest;
use super::context::snapshot_combat;
use super::context::snapshot_event;
use super::context::snapshot_rest_site;
use super::context::snapshot_reward;
use super::context::snapshot_shop;
use super::effect::PyEffectPending;
use super::effect::snapshot_effect_pending;
use super::event::PyEventName;
use super::map::PyMap;
use super::map::snapshot_map;
use super::potion::PyPotion;
use super::potion::snapshot_potion;
use super::relic::PyRelic;
use super::relic::snapshot_relic;

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "GameState",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyGameState {
    // Contexts: None while inactive; all None = Focus::Map
    pub combat: Option<PyCombat>,
    pub reward: Option<PyReward>,
    pub event: Option<PyEvent>,
    pub shop: Option<PyShop>,
    pub rest_site: Option<PyRestSite>,
    pub chest: Option<PyChest>,
    pub game_over: bool,
    pub ascension: u8,
    pub act: u8,
    pub character: PyCharacter,
    pub deck: Vec<PyCard>,
    pub relics: Vec<PyRelic>,
    pub potions: Vec<PyPotion>,
    pub potion_slots_max: u8,
    pub potion_drop_mod: i32, // A combat's potion drop chance is 40 plus this, within 0..=100; a drop lowers it by 10, a miss raises it by 10
    pub unknown_chance_monster: f32, // A "?" Room's odds; the event chance is what the three leave
    pub unknown_chance_shop: f32,
    pub unknown_chance_treasure: f32,
    pub pool_events: Vec<PyEventName>, // Events a "?" can still become, regular then special; each is drawn without replacement
    pub pool_event_special: Vec<PyEventName>,
    pub map: PyMap,
    pub effect_pending: Option<PyEffectPending>,
    pub effect_pending_selected: Vec<PyCard>,
}

// Snapshot builders
pub fn snapshot_state(state: &GameState) -> PyGameState {
    PyGameState {
        combat: state.combat.active.then(|| snapshot_combat(state)),
        reward: state.reward.active.then(|| snapshot_reward(state)),
        event: state.event.active.then(|| snapshot_event(state)),
        shop: state.shop.active.then(|| snapshot_shop(state)),
        rest_site: state.rest_site.active.then(|| snapshot_rest_site(state)),
        chest: state.chest.active.then(|| snapshot_chest(state)),
        game_over: state.game_over,
        ascension: state.ascension,
        act: state.act,
        character: snapshot_character(state),
        deck: state
            .id_card_deck
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        relics: state
            .id_relics
            .iter()
            .flatten()
            .map(|&id| snapshot_relic(id, &state.entities[id]))
            .collect(),
        potions: state
            .id_potions
            .iter()
            .map(|&id| snapshot_potion(id, &state.entities[id]))
            .collect(),
        potion_slots_max: state.potion_slots_max,
        potion_drop_mod: state.potion_drop_mod,
        unknown_chance_monster: state.unknown_chance_monster,
        unknown_chance_shop: state.unknown_chance_shop,
        unknown_chance_treasure: state.unknown_chance_treasure,
        pool_events: state.pool_events.iter().map(|&name| name.into()).collect(),
        pool_event_special: state
            .pool_event_special
            .iter()
            .map(|&name| name.into())
            .collect(),
        map: snapshot_map(state),
        effect_pending: state.effect_pending.as_ref().map(snapshot_effect_pending),
        effect_pending_selected: state
            .effect_pending_selected
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
    }
}
