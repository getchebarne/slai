#![allow(dead_code)]

use pyo3::prelude::*;

mod action;
mod cards;
mod character;
mod consts;
mod effect;
mod engine;
mod entity;
mod ffi;
mod game;
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
use ffi::PyPhase;
use ffi::PyRelic;
use ffi::PyRoom;
use ffi::snapshot_state;
use ffi::to_internal_action;
use game::create_game_state;
use game::step;

// GameEnv

#[pyclass]
struct GameEnv {
    state: game::GameState,
}

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
    const CARDS_DRAWN_PER_TURN: u8 = consts::CARDS_DRAWN_PER_TURN;
    #[classattr]
    const NIGHTMARE_COPIES: u8 = consts::NIGHTMARE_COPIES;
    #[classattr]
    const MAX_BLOCK: u16 = consts::MAX_BLOCK;
    #[classattr]
    const MAP_HEIGHT: usize = consts::MAP_HEIGHT;
    #[classattr]
    const MAP_WIDTH: usize = consts::MAP_WIDTH;

    #[new]
    #[pyo3(signature = (ascension=0))]
    fn new(ascension: u8) -> Self {
        // Placeholder seed; consumers must call `reset(seed=...)` before stepping (gymnasium convention)
        let state = create_game_state(ascension, 0);
        GameEnv { state }
    }

    // Start a fresh run. Returns the initial obs
    #[pyo3(signature = (seed=42))]
    fn reset(&mut self, seed: u64) -> PyGameState {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed);
        snapshot_state(&self.state)
    }

    // Apply an action. Returns `(obs, terminated)`
    fn step(&mut self, action: PyAction) -> PyResult<(PyGameState, bool)> {
        let internal =
            to_internal_action(action).map_err(pyo3::exceptions::PyValueError::new_err)?;
        step(&mut self.state, internal).map_err(pyo3::exceptions::PyValueError::new_err)?;
        let obs = snapshot_state(&self.state);
        let terminated = matches!(self.state.phase, types::Phase::GameOver);
        Ok((obs, terminated))
    }
}

#[pymodule]
fn slai(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEnv>()?;
    m.add_class::<PyActionType>()?;
    m.add_class::<PyAction>()?;
    m.add_class::<PyGameState>()?;
    m.add_class::<PyCard>()?;
    m.add_class::<PyCharacter>()?;
    m.add_class::<PyMonster>()?;
    m.add_class::<PyIntent>()?;
    m.add_class::<PyEnergy>()?;
    m.add_class::<PyMap>()?;
    m.add_class::<PyRoom>()?;
    m.add_class::<PyModifier>()?;
    m.add_class::<PyRelic>()?;
    m.add_class::<ffi::PyPotion>()?;

    // Unit-enum mirrors
    m.add_class::<ffi::PyCardKind>()?;
    m.add_class::<ffi::PyCardColor>()?;
    m.add_class::<ffi::PyCardRarity>()?;
    m.add_class::<ffi::PyCardCostKind>()?;
    m.add_class::<ffi::PyRoomKind>()?;
    m.add_class::<ffi::PyChestKind>()?;
    m.add_class::<ffi::PyPotionName>()?;
    m.add_class::<ffi::PyPotionRarity>()?;
    m.add_class::<ffi::PyModifierKind>()?;
    m.add_class::<ffi::PyIntentKind>()?;
    m.add_class::<ffi::PyCandidatePool>()?;
    m.add_class::<ffi::PyRelicName>()?;
    m.add_class::<ffi::PyRelicTier>()?;
    m.add_class::<ffi::PyCardName>()?;
    m.add_class::<ffi::PyMonsterName>()?;

    // Complex enum mirrors
    m.add_class::<PyPhase>()?;
    m.add_class::<ffi::PySelectionKind>()?;
    m.add_class::<ffi::PyTarget>()?;
    m.add_class::<ffi::PyEffect>()?;
    Ok(())
}
