#![allow(dead_code)]

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::gen_stub_pyclass;
use pyo3_stub_gen::derive::gen_stub_pymethods;

mod action;
mod cards;
mod character;
mod consts;
mod effect;
mod engine;
mod entity;
mod events;
mod ffi;
pub mod game;
mod map;
mod modifier;
mod monsters;
mod potions;
mod relics;
mod types;
mod utils;

use ffi::PyAction;
use ffi::PyActionType;
use ffi::PyCard;
use ffi::PyCharacter;
use ffi::PyEnergy;
use ffi::PyGameState;
use ffi::PyIntent;
use ffi::PyMap;
use ffi::PyModifier;
use ffi::PyMonster;
use ffi::PyRelic;
use ffi::PyRoom;
use ffi::from_internal_action;
use ffi::snapshot_state;
use ffi::to_internal_action;
use game::create_game_state;
use game::step;

// GameEnv

#[gen_stub_pyclass]
#[pyclass(module = "slai.slai")]
struct GameEnv {
    state: game::GameState,
}

#[gen_stub_pymethods]
#[pymethods]
impl GameEnv {
    // Game-shape constants — mirror of `crate::consts` for encoders/wrappers
    #[classattr]
    const MAX_MONSTERS: usize = consts::MAX_MONSTERS;
    #[classattr]
    const MAX_SIZE_HAND: usize = consts::MAX_SIZE_HAND;
    #[classattr]
    const MAX_COMBAT_CARD_REWARD: usize = consts::MAX_COMBAT_CARD_REWARD;
    #[classattr]
    const CARDS_DRAWN_PER_TURN: u16 = consts::CARDS_DRAWN_PER_TURN;
    #[classattr]
    const NIGHTMARE_COPIES: u8 = consts::NIGHTMARE_COPIES;
    #[classattr]
    const MAX_BLOCK: u16 = consts::MAX_BLOCK;
    #[classattr]
    const MAP_HEIGHT: usize = consts::MAP_HEIGHT;
    #[classattr]
    const MAP_WIDTH: usize = consts::MAP_WIDTH;

    #[new]
    #[pyo3(signature = (ascension=0, fast_mode=false))]
    fn new(ascension: u8, fast_mode: bool) -> Self {
        // Placeholder seed; consumers must call `reset(seed=...)` before stepping (gymnasium convention)
        let state = create_game_state(ascension, 0, fast_mode);
        GameEnv { state }
    }

    // Start a fresh run. Returns the initial obs
    #[pyo3(signature = (seed=42))]
    fn reset(&mut self, seed: u64) -> PyGameState {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed, self.state.fast_mode);
        snapshot_state(&self.state)
    }

    // Apply an action. Returns `(obs, terminated)`
    fn step(&mut self, action: PyAction) -> PyResult<(PyGameState, bool)> {
        let internal =
            to_internal_action(action).map_err(pyo3::exceptions::PyValueError::new_err)?;
        step(&mut self.state, internal).map_err(pyo3::exceptions::PyValueError::new_err)?;
        let obs = snapshot_state(&self.state);
        Ok((obs, self.state.game_over))
    }

    // Cached at every settle point; empty when game_over
    fn get_legal_actions(&self) -> Vec<PyAction> {
        self.state
            .legal_actions
            .iter()
            .cloned()
            .map(from_internal_action)
            .collect()
    }
}

#[pymodule]
fn slai(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<GameEnv>()?;
    module.add_class::<PyActionType>()?;
    module.add_class::<PyAction>()?;
    module.add_class::<PyGameState>()?;
    module.add_class::<PyCard>()?;
    module.add_class::<PyCharacter>()?;
    module.add_class::<PyMonster>()?;
    module.add_class::<PyIntent>()?;
    module.add_class::<PyEnergy>()?;
    module.add_class::<PyMap>()?;
    module.add_class::<PyRoom>()?;
    module.add_class::<PyModifier>()?;
    module.add_class::<PyRelic>()?;
    module.add_class::<ffi::PyPotion>()?;

    // Unit-enum mirrors
    module.add_class::<ffi::PyCardKind>()?;
    module.add_class::<ffi::PyCardColor>()?;
    module.add_class::<ffi::PyCardRarity>()?;
    module.add_class::<ffi::PyPlayRestriction>()?;
    module.add_class::<ffi::PyCardCostKind>()?;
    module.add_class::<ffi::PyRoomKind>()?;
    module.add_class::<ffi::PyPotionName>()?;
    module.add_class::<ffi::PyPotionRarity>()?;
    module.add_class::<ffi::PyModifierKind>()?;
    module.add_class::<ffi::PyIntentKind>()?;
    module.add_class::<ffi::PyCandidatePool>()?;
    module.add_class::<ffi::PyRelicName>()?;
    module.add_class::<ffi::PyRelicTier>()?;
    module.add_class::<ffi::PyCardName>()?;
    module.add_class::<ffi::PyMonsterName>()?;
    module.add_class::<ffi::PyMonsterEncounter>()?;
    module.add_class::<ffi::PyEventName>()?;
    module.add_class::<ffi::PyCandidatePoolCardFilter>()?;
    module.add_class::<ffi::PyCandidatePoolMonstersFilter>()?;
    module.add_class::<ffi::PyScreen>()?;
    module.add_class::<ffi::PyEvent>()?;
    module.add_class::<ffi::PyEventOption>()?;

    // Complex enum mirrors
    module.add_class::<ffi::PySelectionKind>()?;
    module.add_class::<ffi::PyTarget>()?;
    module.add_class::<ffi::PyEffect>()?;
    module.add_class::<ffi::PyDeltaSign>()?;
    module.add_class::<ffi::PyAmount>()?;

    // Reward + Shop surface
    module.add_class::<ffi::PyReward>()?;
    module.add_class::<ffi::PyShop>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
