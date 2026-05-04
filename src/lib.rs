#![allow(dead_code)]

use pyo3::prelude::*;
use pyo3::types::PyDict;

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
mod types;
mod utils;

use ffi::{
    Action, Card, Character, Energy, GameState, Intent, Map, MapNode, Modifier, Monster, Phase,
    build_view,
};
use game::{create_game_state, step};

// ---- GameEnv ----

#[pyclass]
struct GameEnv {
    state: game::GameState,
}

#[pymethods]
impl GameEnv {
    #[new]
    #[pyo3(signature = (ascension=0))]
    fn new(ascension: u8) -> Self {
        // State is created with a placeholder seed; consumers must call
        // `reset(seed=...)` before stepping (gymnasium convention)
        let state = create_game_state(ascension, 0);
        GameEnv { state }
    }

    /// Start a fresh run. Returns `(obs, info)`.
    #[pyo3(signature = (seed=42))]
    fn reset<'py>(&mut self, py: Python<'py>, seed: u64) -> (GameState, Bound<'py, PyDict>) {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed);
        (build_view(&self.state), PyDict::new(py))
    }

    /// Apply an action. Returns `(obs, reward, terminated, truncated, info)`.
    /// `reward` is currently always 0.0 — no reward function is defined yet.
    /// `truncated` is currently always false — there's no step-limit truncation.
    fn step<'py>(
        &mut self,
        py: Python<'py>,
        action: Action,
    ) -> PyResult<(GameState, f32, bool, bool, Bound<'py, PyDict>)> {
        step(&mut self.state, action.into())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let obs = build_view(&self.state);
        let terminated = matches!(self.state.phase, types::Phase::GameOver);
        let truncated = false;
        let reward = 0.0_f32;
        Ok((obs, reward, terminated, truncated, PyDict::new(py)))
    }
}

// Module
#[pymodule]
fn slai(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEnv>()?;
    m.add_class::<Action>()?;
    m.add_class::<GameState>()?;
    m.add_class::<Card>()?;
    m.add_class::<Character>()?;
    m.add_class::<Monster>()?;
    m.add_class::<Intent>()?;
    m.add_class::<Energy>()?;
    m.add_class::<Map>()?;
    m.add_class::<MapNode>()?;
    m.add_class::<Modifier>()?;
    // Unit-enum mirrors
    m.add_class::<ffi::CardKind>()?;
    m.add_class::<ffi::CardColor>()?;
    m.add_class::<ffi::CardRarity>()?;
    m.add_class::<ffi::CardCostKind>()?;
    m.add_class::<ffi::RoomKind>()?;
    m.add_class::<ffi::ModifierKind>()?;
    m.add_class::<ffi::CandidatePool>()?;
    // Complex enum mirrors
    m.add_class::<Phase>()?;
    m.add_class::<ffi::Selection>()?;
    m.add_class::<ffi::Target>()?;
    m.add_class::<ffi::Effect>()?;
    Ok(())
}
