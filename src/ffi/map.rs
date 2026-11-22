use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;

use super::macros::flat_variants;
use super::macros::mirror_enum;

use crate::game::GameState;
use crate::map::edge_indices;
use crate::types::IdRooms;
use crate::types::Location;
use crate::types::RoomKind;

mirror_enum!(PyRoomKind from RoomKind, "RoomKind", {
    CombatMonster, CombatElite, CombatBoss, RestSite, Treasure, EventRoom, Shop, Unknown,
});

flat_variants!(PyLocation as "Location" {
    Start => PyLocationStart as "LocationStart",
    Overworld => PyLocationOverworld as "LocationOverworld" { y: usize, x: usize },
    BossRoom => PyLocationBossRoom as "LocationBossRoom",
    BossTreasure => PyLocationBossTreasure as "LocationBossTreasure",
    Victory => PyLocationVictory as "LocationVictory",
});

impl From<Location> for PyLocation {
    fn from(location: Location) -> Self {
        match location {
            Location::Start => Self::Start(PyLocationStart),
            Location::Overworld { y, x } => Self::Overworld(PyLocationOverworld { y, x }),
            Location::BossRoom => Self::BossRoom(PyLocationBossRoom),
            Location::BossTreasure => Self::BossTreasure(PyLocationBossTreasure),
            Location::Victory => Self::Victory(PyLocationVictory),
        }
    }
}

#[pyclass(frozen, get_all, name = "Room", module = "slai._slai")]
#[derive(Debug)]
pub struct PyRoom {
    pub id: usize,
    pub room_kind: PyRoomKind,
    pub edges: Vec<usize>,
    pub burning: bool, // The act's burning elite, shown from the map's creation to the act's end

    // Its row and column in the Map's `id_rooms`
    pub y: usize,
    pub x: usize,
}

#[pyclass(frozen, get_all, name = "Map", module = "slai._slai")]
pub struct PyMap {
    pub id_rooms: PyIdRooms,
    pub location: PyLocation,

    // The current Room's resolved kind ("?" stays Unknown on its Room); no Shop "?" after a Shop
    pub room_kind_resolved: Option<PyRoomKind>,
    pub identity_hash: u64,
}

// The Room id grid, made into its Python lists on the first read; every read returns the same lists
pub struct PyIdRooms {
    id_rooms: IdRooms,
    object: PyOnceLock<Py<PyAny>>,
}

// Typed as the grid's lists; the field's getter converts by reference
impl<'py> IntoPyObject<'py> for PyIdRooms {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    const OUTPUT_TYPE: PyStaticExpr = <IdRooms as IntoPyObject<'py>>::OUTPUT_TYPE;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self)
            .into_pyobject(py)
            .map(|id_rooms| id_rooms.to_owned())
    }
}

impl<'a, 'py> IntoPyObject<'py> for &'a PyIdRooms {
    type Target = PyAny;
    type Output = Borrowed<'a, 'py, PyAny>;
    type Error = PyErr;
    const OUTPUT_TYPE: PyStaticExpr = <IdRooms as IntoPyObject<'py>>::OUTPUT_TYPE;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let id_rooms = self
            .object
            .get_or_try_init(py, || self.id_rooms.into_pyobject(py).map(Bound::unbind))?;
        Ok(id_rooms.bind_borrowed(py))
    }
}

// Position-independent hash of the Room topology (kinds, edges and the burning elite's mark) — a stable map identity for the
// RL encoder's static-grid cache. Excludes the live position so it's constant across a map's life.
fn map_identity_hash(state: &GameState) -> u64 {
    use std::hash::Hash;
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for (y, row) in state.map.id_rooms.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let Some(id_room) = *cell {
                let room = &state.entities[id_room];
                (y, x, room.room_kind, room.room_edges, room.room_burning).hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

pub(crate) fn snapshot_map(state: &GameState) -> PyMap {
    PyMap {
        id_rooms: PyIdRooms {
            id_rooms: state.map.id_rooms,
            object: PyOnceLock::new(),
        },
        location: state.map.location.into(),
        room_kind_resolved: state.map.room_kind_resolved.map(Into::into),
        identity_hash: map_identity_hash(state),
    }
}

pub(crate) fn snapshot_room(state: &GameState, id_room: usize) -> PyRoom {
    let room = &state.entities[id_room];
    PyRoom {
        id: id_room,
        room_kind: room.room_kind.into(),
        edges: edge_indices(room.room_edges).collect(),
        burning: room.room_burning,
        y: room.room_y,
        x: room.room_x,
    }
}
