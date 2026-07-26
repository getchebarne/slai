use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::SelectionKind;

use super::macros::variant_union;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "CandidatePoolHand",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolHand {
    pub filter: PyCandidatePoolCardFilter,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CandidatePoolCharacter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolCharacter;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "CandidatePoolMonsters",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolMonsters {
    pub filter: PyCandidatePoolMonstersFilter,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CandidatePoolSource",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolSource;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CandidatePoolDiscover",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolDiscover;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "CandidatePoolDeck",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolDeck {
    pub filter: PyCandidatePoolCardFilter,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CandidatePoolEventPickCard",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolEventPickCard;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "CandidatePoolEventPickPotion",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyCandidatePoolEventPickPotion;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyCandidatePool {
    Hand(PyCandidatePoolHand),
    Character(PyCandidatePoolCharacter),
    Monsters(PyCandidatePoolMonsters),
    Source(PyCandidatePoolSource),
    Discover(PyCandidatePoolDiscover),
    Deck(PyCandidatePoolDeck),
    EventPickCard(PyCandidatePoolEventPickCard),
    EventPickPotion(PyCandidatePoolEventPickPotion),
}

variant_union!(PyCandidatePool {
    Hand => PyCandidatePoolHand,
    Character => PyCandidatePoolCharacter,
    Monsters => PyCandidatePoolMonsters,
    Source => PyCandidatePoolSource,
    Discover => PyCandidatePoolDiscover,
    Deck => PyCandidatePoolDeck,
    EventPickCard => PyCandidatePoolEventPickCard,
    EventPickPotion => PyCandidatePoolEventPickPotion,
});

impl From<CandidatePool> for PyCandidatePool {
    fn from(pool: CandidatePool) -> Self {
        match pool {
            CandidatePool::Hand { filter } => Self::Hand(PyCandidatePoolHand {
                filter: filter.into(),
            }),
            CandidatePool::Character => Self::Character(PyCandidatePoolCharacter),
            CandidatePool::Monsters { filter } => Self::Monsters(PyCandidatePoolMonsters {
                filter: filter.into(),
            }),
            CandidatePool::Source => Self::Source(PyCandidatePoolSource),
            CandidatePool::Discover => Self::Discover(PyCandidatePoolDiscover),
            CandidatePool::Deck { filter } => Self::Deck(PyCandidatePoolDeck {
                filter: filter.into(),
            }),
            CandidatePool::EventPickCard => Self::EventPickCard(PyCandidatePoolEventPickCard),
            CandidatePool::EventPickPotion => Self::EventPickPotion(PyCandidatePoolEventPickPotion),
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "CandidatePoolMonstersFilter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCandidatePoolMonstersFilter {
    All,
    Other,
    Picked,
}

impl From<CandidatePoolMonstersFilter> for PyCandidatePoolMonstersFilter {
    fn from(f: CandidatePoolMonstersFilter) -> Self {
        match f {
            CandidatePoolMonstersFilter::All => Self::All,
            CandidatePoolMonstersFilter::Other => Self::Other,
            CandidatePoolMonstersFilter::Picked => Self::Picked,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "CandidatePoolCardFilter",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyCandidatePoolCardFilter {
    Purgeable,
    Upgradeable,
    Any,
    Transformable,
    PurgeableCurse,
}

impl From<CandidatePoolCardFilter> for PyCandidatePoolCardFilter {
    fn from(f: CandidatePoolCardFilter) -> Self {
        match f {
            CandidatePoolCardFilter::Purgeable => Self::Purgeable,
            CandidatePoolCardFilter::Upgradeable => Self::Upgradeable,
            CandidatePoolCardFilter::Any => Self::Any,
            CandidatePoolCardFilter::Transformable => Self::Transformable,
            CandidatePoolCardFilter::PurgeableCurse => Self::PurgeableCurse,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "SelectionKindAll",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindAll;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "SelectionKindSingle",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindSingle;

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "SelectionKindRandom",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindRandom {
    pub count: u8,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "SelectionKindInput",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PySelectionKindInput {
    pub count: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PySelectionKind {
    All(PySelectionKindAll),
    Single(PySelectionKindSingle),
    Random(PySelectionKindRandom),
    Input(PySelectionKindInput),
}

variant_union!(PySelectionKind {
    All => PySelectionKindAll,
    Single => PySelectionKindSingle,
    Random => PySelectionKindRandom,
    Input => PySelectionKindInput,
});

impl From<SelectionKind> for PySelectionKind {
    fn from(selection_kind: SelectionKind) -> Self {
        match selection_kind {
            SelectionKind::All => Self::All(PySelectionKindAll),
            SelectionKind::Single => Self::Single(PySelectionKindSingle),
            SelectionKind::Random { count } => Self::Random(PySelectionKindRandom { count }),
            SelectionKind::Input { count } => Self::Input(PySelectionKindInput { count }),
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "Target",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyTarget {
    pub candidate_pool: PyCandidatePool,
    pub selection_kind: PySelectionKind,
}
