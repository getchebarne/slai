use pyo3::prelude::*;

use crate::game::GameState;

use super::modifier::PyModifier;
use super::modifier::snapshot_modifiers;

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "Character",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyCharacter {
    pub name: String,
    pub health: u16,
    pub health_max: u16,
    pub block: u16,
    pub modifiers: Vec<PyModifier>,
    pub gold: u16,
}

#[pymethods]
impl PyCharacter {
    #[new]
    fn new(
        name: String,
        health: u16,
        health_max: u16,
        block: u16,
        modifiers: Vec<PyModifier>,
        gold: u16,
    ) -> Self {
        Self {
            name,
            health,
            health_max,
            block,
            modifiers,
            gold,
        }
    }
}

pub(crate) fn snapshot_character(state: &GameState) -> PyCharacter {
    let character = &state.entities[state.id_character];
    PyCharacter {
        name: character.character_name.to_string(),
        health: character.vitals.health,
        health_max: character.vitals.health_max,
        block: character.vitals.block,
        modifiers: snapshot_modifiers(&character.modifiers),
        gold: character.character_gold,
    }
}
