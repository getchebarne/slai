use pyo3::prelude::*;

use crate::game::GameState;
use crate::relics::iter_owned_relics;

use super::card::PyCard;
use super::card::snapshot_card;
use super::character::PyCharacter;
use super::character::snapshot_character;
use super::effect::PyEffect;
use super::effect::snapshot_effect;
use super::map::PyMap;
use super::map::snapshot_map;
use super::mode::PyMode;
use super::mode::snapshot_mode;
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
    pub mode: PyMode,
    pub game_over: bool,
    pub ascension: u8,
    pub character: PyCharacter,
    pub deck: Vec<PyCard>,
    pub relics: Vec<PyRelic>,
    // Slot-indexed belt (length potion_slots_max); None at empty slots so positions stay valid
    pub potions: Vec<Option<PyPotion>>,
    pub potion_slots_max: u8,
    pub map: PyMap,
    // Halt-for-input is orthogonal to mode
    pub pending: Option<PyEffect>,
}

// Snapshot builders
pub fn snapshot_state(state: &GameState) -> PyGameState {
    PyGameState {
        mode: snapshot_mode(state),
        game_over: state.game_over,
        ascension: state.ascension,
        character: snapshot_character(state),
        deck: state
            .id_deck
            .iter()
            .map(|&id| snapshot_card(state, id))
            .collect(),
        relics: iter_owned_relics(&state.id_relics)
            .map(|(_name, id)| snapshot_relic(&state.entities[id]))
            .collect(),
        potions: state.id_potions[..state.potion_slots_max as usize]
            .iter()
            .map(|s| s.map(|id| snapshot_potion(&state.entities[id])))
            .collect(),
        potion_slots_max: state.potion_slots_max,
        map: snapshot_map(state),
        pending: state.effect_pending.as_ref().map(snapshot_effect),
    }
}
