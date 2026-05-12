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
mod relics;
mod types;
mod utils;

use ffi::{
    Action, ActionType, Card, Character, Energy, GameState, Intent, Map, MapNode, Modifier,
    Monster, Phase, Relic, build_view,
};
use game::{create_game_state, step};

// ---- GameEnv ----

#[pyclass]
struct GameEnv {
    state: game::GameState,
}

#[pymethods]
impl GameEnv {
    // ---- Game-shape constants ----
    //
    // Mirror of `crate::consts` values that consumers (RL encoders,
    // gym wrappers, dataset builders) need at module load. Exposed as
    // class attributes so callers don't need to import a separate
    // `slai.consts` namespace and so the values travel with the env.
    //
    // Deliberate omissions: deck / draw / discard pile sizes are
    // *unbounded* in the engine — those caps are encoder concerns and
    // belong on the consumer side, not here.
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
        let internal = action
            .try_into()
            .map_err(|e: String| pyo3::exceptions::PyValueError::new_err(e))?;
        step(&mut self.state, internal).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let obs = build_view(&self.state);
        let terminated = matches!(self.state.phase, types::Phase::GameOver);
        let truncated = false;
        let reward = 0.0_f32;
        Ok((obs, reward, terminated, truncated, PyDict::new(py)))
    }

    // Remove once shop / event distribution channels exist.
    //
    // Accepts the discriminant as `u8` so users can pass either the PyO3
    // `RelicName` (it has __int__) or the Python IntEnum shim (it is an int).
    fn dev_grant_relic(&mut self, name: u8) -> PyResult<()> {
        let internal = types::RelicName::from_u8(name);
        if self.state.id_relics[internal as usize].is_some() {
            return Ok(());
        }
        let id = self.state.entities.len();
        self.state.entities.push(relics::get_relic(internal));
        self.state.id_relics[internal as usize] = Some(id);
        Ok(())
    }
}

// Module
#[pymodule]
fn slai(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEnv>()?;
    m.add_class::<ActionType>()?;
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
    m.add_class::<Relic>()?;
    // Unit-enum mirrors
    m.add_class::<ffi::CardKind>()?;
    m.add_class::<ffi::CardColor>()?;
    m.add_class::<ffi::CardRarity>()?;
    m.add_class::<ffi::CardCostKind>()?;
    m.add_class::<ffi::RoomKind>()?;
    m.add_class::<ffi::ChestKind>()?;
    m.add_class::<ffi::ModifierKind>()?;
    m.add_class::<ffi::IntentKind>()?;
    m.add_class::<ffi::CandidatePool>()?;
    m.add_class::<ffi::RelicName>()?;
    m.add_class::<ffi::RelicTier>()?;
    m.add_class::<ffi::CardName>()?;
    m.add_class::<ffi::MonsterName>()?;
    // Complex enum mirrors
    m.add_class::<Phase>()?;
    m.add_class::<ffi::Selection>()?;
    m.add_class::<ffi::Target>()?;
    m.add_class::<ffi::Effect>()?;
    Ok(())
}
