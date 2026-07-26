use pyo3::prelude::*;

use crate::consts::MAP_HEIGHT;
use crate::game::GameState;
use crate::game::Location;
use crate::map::edge_indices;
use crate::types::RoomKind;

use super::monster::PyMonsterEncounter;

#[pyclass(
    from_py_object,
    eq,
    eq_int,
    frozen,
    name = "RoomKind",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyRoomKind {
    CombatMonster,
    CombatElite,
    CombatBoss,
    RestSite,
    Treasure,
    EventRoom,
    Shop,
    Unknown,
}

impl From<RoomKind> for PyRoomKind {
    fn from(kind: RoomKind) -> Self {
        match kind {
            RoomKind::CombatMonster => Self::CombatMonster,
            RoomKind::CombatElite => Self::CombatElite,
            RoomKind::CombatBoss => Self::CombatBoss,
            RoomKind::RestSite => Self::RestSite,
            RoomKind::Treasure => Self::Treasure,
            RoomKind::EventRoom => Self::EventRoom,
            RoomKind::Shop => Self::Shop,
            RoomKind::Unknown => Self::Unknown,
        }
    }
}

#[pyclass(from_py_object, frozen, get_all, name = "Room", module = "slai.slai")]
#[derive(Debug, Clone)]
pub struct PyRoom {
    pub room_kind: PyRoomKind,
    pub edges: Vec<usize>,
    pub chest_opened: bool,
}

#[pymethods]
impl PyRoom {
    #[new]
    fn new(room_kind: PyRoomKind, edges: Vec<usize>, chest_opened: bool) -> Self {
        Self {
            room_kind,
            edges,
            chest_opened,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    frozen,
    get_all,
    name = "Map",
    module = "slai.slai"
)]
#[derive(Debug, Clone)]
pub struct PyMap {
    pub rooms: Vec<Vec<Option<PyRoom>>>,
    pub y_current: Option<usize>,
    pub x_current: Option<usize>,
    pub boss: PyMonsterEncounter,
    pub identity_hash: u64,
}

#[pymethods]
impl PyMap {
    #[new]
    fn new(
        rooms: Vec<Vec<Option<PyRoom>>>,
        y_current: Option<usize>,
        x_current: Option<usize>,
        boss: PyMonsterEncounter,
        identity_hash: u64,
    ) -> Self {
        Self {
            rooms,
            y_current,
            x_current,
            boss,
            identity_hash,
        }
    }
}

// Position-independent hash of the room topology (kinds + edges) — a stable map identity for the
// RL encoder's static-grid cache. Excludes the live position so it's constant across a map's life.
fn map_identity_hash(state: &GameState) -> u64 {
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for (y, row) in state.id_rooms.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(id_room) = *cell {
                let room = &state.entities[id_room];
                (y, x, room.room_kind, room.room_edges).hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

pub(crate) fn snapshot_map(state: &GameState) -> PyMap {
    let rooms = state
        .id_rooms
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| {
                    cell.map(|id_room| {
                        let room = &state.entities[id_room];
                        PyRoom {
                            room_kind: room.room_kind.into(),
                            edges: edge_indices(room.room_edges).collect(),
                            chest_opened: room.room_chest_opened,
                        }
                    })
                })
                .collect()
        })
        .collect();

    let (y_current, x_current) = match state.location {
        Location::Start => (None, None),
        Location::Overworld { y, x } => (Some(y), Some(x)),
        Location::BossRoom => (Some(MAP_HEIGHT), Some(0)),
    };
    PyMap {
        rooms,
        y_current,
        x_current,
        boss: state.encounter_boss.into(),
        identity_hash: map_identity_hash(state),
    }
}
