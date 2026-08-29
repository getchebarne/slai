use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::SelectionKind;

use super::macros::flat_variants;
use super::macros::mirror_enum;

mirror_enum!(PyCandidatePool from CandidatePool, "CandidatePool", {
    Hand, Character, Monsters, Source, Discover, Deck, PileDraw, PileDiscard, PileExhaust,
    EventRollCard, EventRollRelic, EventRollPotion,
});

mirror_enum!(PyCandidateFilter from CandidateFilter, "CandidateFilter", {
    Any, Purgeable, Upgradeable, Transformable, PurgeableCurse, KindAttack, KindSkill,
    KindPower, Costed, Picked, NotSource, NotMinion, StarterStrike, StarterUpgradeable,
});

flat_variants!(PySelectionKind {
    All => PySelectionKindAll as "SelectionKindAll",
    Single => PySelectionKindSingle as "SelectionKindSingle",
    Random => PySelectionKindRandom as "SelectionKindRandom" { count: u8 },
    Input => PySelectionKindInput as "SelectionKindInput" { count: u16 },
    InputUpTo => PySelectionKindInputUpTo as "SelectionKindInputUpTo" { count: u16 },
});

impl From<SelectionKind> for PySelectionKind {
    fn from(selection_kind: SelectionKind) -> Self {
        match selection_kind {
            SelectionKind::All => Self::All(PySelectionKindAll),
            SelectionKind::Single => Self::Single(PySelectionKindSingle),
            SelectionKind::Random { count } => Self::Random(PySelectionKindRandom { count }),
            SelectionKind::Input { count } => Self::Input(PySelectionKindInput { count }),
            SelectionKind::InputUpTo { count } => {
                Self::InputUpTo(PySelectionKindInputUpTo { count })
            }
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
    pub filter: PyCandidateFilter,
    pub selection_kind: PySelectionKind,
}
