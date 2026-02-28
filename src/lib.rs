#![allow(dead_code)]

use pyo3::prelude::*;

mod action;
mod cards;
mod character;
mod consts;
mod effect;
mod engine;
mod game;
mod map;
mod modifier;
mod monsters;
mod state;
mod types;
mod utils;
mod view;

use action::Action;
use game::{create_game_state, initialize, step};
use view::{
    ViewCard, ViewCharacter, ViewEffectTemplate, ViewEnergy, ViewGameState, ViewIntent, ViewMap,
    ViewMapNode, ViewModifier, ViewMonster, build_view,
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
        Self { hand_idx, monster_idx }
    }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionEndTurn;

#[pymethods]
impl ActionEndTurn {
    #[new]
    fn new() -> Self { Self }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardDiscard {
    #[pyo3(get)]
    hand_idx: usize,
}

#[pymethods]
impl ActionCardDiscard {
    #[new]
    fn new(hand_idx: usize) -> Self { Self { hand_idx } }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionMapNodeSelect {
    #[pyo3(get)]
    column: usize,
}

#[pymethods]
impl ActionMapNodeSelect {
    #[new]
    fn new(column: usize) -> Self { Self { column } }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardRewardSelect {
    #[pyo3(get)]
    reward_idx: usize,
}

#[pymethods]
impl ActionCardRewardSelect {
    #[new]
    fn new(reward_idx: usize) -> Self { Self { reward_idx } }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionCardRewardSkip;

#[pymethods]
impl ActionCardRewardSkip {
    #[new]
    fn new() -> Self { Self }
}

#[pyclass(frozen)]
#[derive(Clone)]
struct ActionRest;

#[pymethods]
impl ActionRest {
    #[new]
    fn new() -> Self { Self }
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
    fn new(deck_idx: usize) -> Self { Self { deck_idx } }
}

// ---- Decode PyAny -> Action ----

fn decode_action(py_action: &Bound<'_, PyAny>) -> PyResult<Action> {
    if let Ok(a) = py_action.extract::<ActionCardPlay>() {
        return Ok(Action::CardPlay { hand_idx: a.hand_idx, monster_idx: a.monster_idx });
    }
    if py_action.extract::<ActionEndTurn>().is_ok() {
        return Ok(Action::EndTurn);
    }
    if let Ok(a) = py_action.extract::<ActionCardDiscard>() {
        return Ok(Action::CardDiscard { hand_idx: a.hand_idx });
    }
    if let Ok(a) = py_action.extract::<ActionMapNodeSelect>() {
        return Ok(Action::MapNodeSelect { column: a.column });
    }
    if let Ok(a) = py_action.extract::<ActionCardRewardSelect>() {
        return Ok(Action::CardRewardSelect { reward_idx: a.reward_idx });
    }
    if py_action.extract::<ActionCardRewardSkip>().is_ok() {
        return Ok(Action::CardRewardSkip);
    }
    if py_action.extract::<ActionRest>().is_ok() {
        return Ok(Action::Rest);
    }
    if let Ok(a) = py_action.extract::<ActionCardUpgrade>() {
        return Ok(Action::CardUpgrade { deck_idx: a.deck_idx });
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
        let mut state = create_game_state(ascension, seed);
        initialize(&mut state);
        GameEnv { state }
    }

    fn get_view(&self) -> ViewGameState {
        build_view(&self.state)
    }

    fn step(&mut self, action: &Bound<'_, PyAny>) -> PyResult<(ViewGameState, bool)> {
        let action = decode_action(action)?;
        step(&mut self.state, action)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let done = self.state.phase == types::Phase::GameOver;
        Ok((build_view(&self.state), done))
    }

    fn reset(&mut self, seed: u64) -> ViewGameState {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed);
        initialize(&mut self.state);
        build_view(&self.state)
    }

    fn phase(&self) -> u8 {
        self.state.phase as u8
    }

    fn phase_name(&self) -> String {
        format!("{:?}", self.state.phase)
    }
}

// ---- Module ----

#[pymodule]
fn slai(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEnv>()?;
    m.add_class::<ActionCardPlay>()?;
    m.add_class::<ActionEndTurn>()?;
    m.add_class::<ActionCardDiscard>()?;
    m.add_class::<ActionMapNodeSelect>()?;
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
    m.add_class::<ViewEffectTemplate>()?;
    Ok(())
}
