#![allow(dead_code)]

use pyo3::prelude::*;

mod types;
mod modifier;
mod effect;
mod state;
mod card;
mod monster;
mod map;
mod process;
mod action;
mod game;
mod view;

use action::Action;
use game::{create_game_state, initialize, step};
use view::{
    ViewGameState, ViewCard, ViewCharacter, ViewMonster, ViewIntent,
    ViewEnergy, ViewMap, ViewMapNode, ViewModifier, ViewEffectTemplate,
    build_view,
};

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

    fn step(&mut self, action_type: u8, action_index: i32) -> (ViewGameState, bool) {
        let action = decode_action(action_type, action_index);
        step(&mut self.state, action);
        let done = self.state.fsm == types::Fsm::GameOver;
        (build_view(&self.state), done)
    }

    fn reset(&mut self, seed: u64) -> ViewGameState {
        let asc = self.state.ascension;
        self.state = create_game_state(asc, seed);
        initialize(&mut self.state);
        build_view(&self.state)
    }

    fn fsm(&self) -> u8 {
        self.state.fsm as u8
    }

    fn fsm_name(&self) -> String {
        format!("{:?}", self.state.fsm)
    }
}

fn decode_action(action_type: u8, action_index: i32) -> Action {
    match action_type {
        0 => Action::PlayCard { hand_idx: action_index as usize },
        1 => Action::EndTurn,
        2 => Action::SelectMonster { monster_idx: action_index as u8 },
        3 => Action::SelectMapNode { column: action_index as usize },
        4 => Action::SelectCardReward { reward_idx: action_index as usize },
        5 => Action::SkipCardReward,
        6 => Action::Rest,
        7 => Action::Upgrade { deck_idx: action_index as usize },
        _ => panic!("Unknown action type: {action_type}"),
    }
}

#[pymodule]
fn slai(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<GameEnv>()?;
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
