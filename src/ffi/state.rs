use pyo3::prelude::*;

use crate::game::GameState;
use crate::relics::iter_owned_relics;

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
use super::effect::PyPendingEffect;
use super::effect::snapshot_pending_effect;
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
    // Slot-indexed belt (length potion_slots_max); None at empty slots so positions stay valid
    pub potions: Vec<Option<PyPotion>>,
    pub potion_slots_max: u8,
    pub map: PyMap,
    // Halt-for-input is orthogonal to the contexts
    pub pending: Option<PyPendingEffect>,
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
        relics: iter_owned_relics(&state.id_relics)
            .map(|(_name, id)| snapshot_relic(id, &state.entities[id]))
            .collect(),
        potions: state.id_potions[..state.potion_slots_max as usize]
            .iter()
            .map(|slot| slot.map(|id| snapshot_potion(id, &state.entities[id])))
            .collect(),
        potion_slots_max: state.potion_slots_max,
        map: snapshot_map(state),
        pending: state.effect_pending.as_ref().map(snapshot_pending_effect),
    }
}
