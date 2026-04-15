#![allow(dead_code)]

use pyo3::prelude::*;

mod action;
mod cards;
mod character;
mod consts;
mod effect;
mod engine;
mod entity;
mod game;
mod map;
mod modifier;
mod monsters;
mod state;
mod types;
mod utils;
mod view;

use action::Action;
use game::{create_game_state, step};
use view::{
    ViewCard, ViewCharacter, ViewEnergy, ViewGameState, ViewIntent, ViewMap, ViewMapNode,
    ViewModifier, ViewMonster, build_view,
};

// ---- Python action classes ----

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardPlay {
    #[pyo3(get)]
    hand_idx: usize,
    #[pyo3(get)]
    monster_idx: Option<usize>,
}

#[pymethods]
impl ActionCardPlay {
    #[new]
    #[pyo3(signature = (hand_idx, monster_idx=None))]
    fn new(hand_idx: usize, monster_idx: Option<usize>) -> Self {
        Self {
            hand_idx,
            monster_idx,
        }
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionEndTurn;

#[pymethods]
impl ActionEndTurn {
    #[new]
    fn new() -> Self {
        Self
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardDiscard {
    #[pyo3(get)]
    indices: Vec<usize>,
}

#[pymethods]
impl ActionCardDiscard {
    #[new]
    fn new(indices: Vec<usize>) -> Self {
        Self { indices }
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionRoomSelect {
    #[pyo3(get)]
    column: usize,
}

#[pymethods]
impl ActionRoomSelect {
    #[new]
    fn new(column: usize) -> Self {
        Self { column }
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardRewardSelect {
    #[pyo3(get)]
    idx_reward: usize,
}

#[pymethods]
impl ActionCardRewardSelect {
    #[new]
    fn new(idx_reward: usize) -> Self {
        Self { idx_reward }
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardRewardSkip;

#[pymethods]
impl ActionCardRewardSkip {
    #[new]
    fn new() -> Self {
        Self
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionRest;

#[pymethods]
impl ActionRest {
    #[new]
    fn new() -> Self {
        Self
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardUpgrade {
    #[pyo3(get)]
    deck_idx: usize,
}

#[pymethods]
impl ActionCardUpgrade {
    #[new]
    fn new(deck_idx: usize) -> Self {
        Self { deck_idx }
    }
}

// ---- Decode PyAny -> Action ----

fn decode_action(py_action: &Bound<'_, PyAny>) -> PyResult<Action> {
    if let Ok(a) = py_action.extract::<ActionCardPlay>() {
        return Ok(Action::CardPlay {
            idx_hand: a.hand_idx,
            idx_monster: a.monster_idx,
        });
    }
    if py_action.extract::<ActionEndTurn>().is_ok() {
        return Ok(Action::EndTurn);
    }
    if let Ok(a) = py_action.extract::<ActionCardDiscard>() {
        return Ok(Action::CardDiscard {
            idx_hand: a.indices,
        });
    }
    if let Ok(a) = py_action.extract::<ActionRoomSelect>() {
        return Ok(Action::RoomSelect {
            idx_column: a.column,
        });
    }
    if let Ok(a) = py_action.extract::<ActionCardRewardSelect>() {
        return Ok(Action::CardRewardSelect {
            idx_reward: a.idx_reward,
        });
    }
    if py_action.extract::<ActionCardRewardSkip>().is_ok() {
        return Ok(Action::CardRewardSkip);
    }
    if py_action.extract::<ActionRest>().is_ok() {
        return Ok(Action::RestSiteRest);
    }
    if let Ok(a) = py_action.extract::<ActionCardUpgrade>() {
        return Ok(Action::RestSiteCardUpgrade {
            idx_deck: a.deck_idx,
        });
    }
    Err(pyo3::exceptions::PyTypeError::new_err(format!(
        "Unknown action type: {}",
        py_action.get_type().name()?
    )))
}

// ---- GameEnv ----

#[pyclass]
struct GameEnv {
    state: state::GameState,
}

#[pymethods]
impl GameEnv {
    #[new]
    #[pyo3(signature = (ascension=0, seed=42))]
    fn new(ascension: u8, seed: u64) -> Self {
        let state = create_game_state(ascension, seed);
        GameEnv { state }
    }

    fn get_view(&self, py: Python<'_>) -> ViewGameState {
        build_view(py, &self.state)
    }

    fn step(
        &mut self,
        py: Python<'_>,
        action: &Bound<'_, PyAny>,
    ) -> PyResult<(ViewGameState, bool)> {
        let action = decode_action(action)?;
        step(&mut self.state, action).map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let done = self.state.phase == types::Phase::GameOver;
        Ok((build_view(py, &self.state), done))
    }

    fn reset(&mut self, py: Python<'_>, seed: u64) -> ViewGameState {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed);
        build_view(py, &self.state)
    }

    fn phase_name(&self) -> String {
        view::phase_variant_name(self.state.phase)
    }
}

// Module
#[pymodule]
fn slai(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEnv>()?;
    m.add_class::<ActionCardPlay>()?;
    m.add_class::<ActionEndTurn>()?;
    m.add_class::<ActionCardDiscard>()?;
    m.add_class::<ActionRoomSelect>()?;
    m.add_class::<ActionCardRewardSelect>()?;
    m.add_class::<ActionCardRewardSkip>()?;
    m.add_class::<ActionRest>()?;
    m.add_class::<ActionCardUpgrade>()?;
    m.add_class::<ViewGameState>()?;
    m.add_class::<ViewCard>()?;
    m.add_class::<ViewCharacter>()?;
    m.add_class::<ViewMonster>()?;
    m.add_class::<ViewIntent>()?;
    m.add_class::<ViewEnergy>()?;
    m.add_class::<ViewMap>()?;
    m.add_class::<ViewMapNode>()?;
    m.add_class::<ViewModifier>()?;
    // Selection variants
    m.add_class::<view::ViewSelectionAll>()?;
    m.add_class::<view::ViewSelectionRandom>()?;
    m.add_class::<view::ViewSelectionInput>()?;
    // Effect variants
    m.add_class::<view::ViewDamagePhysical>()?;
    m.add_class::<view::ViewBlockGain>()?;
    m.add_class::<view::ViewModifierGain>()?;
    m.add_class::<view::ViewModifierRemove>()?;
    m.add_class::<view::ViewEnergyGain>()?;
    m.add_class::<view::ViewAddShivs>()?;
    m.add_class::<view::ViewCardDraw>()?;
    m.add_class::<view::ViewCardDiscard>()?;
    m.add_class::<view::ViewCalculatedGamble>()?;
    Ok(())
}
