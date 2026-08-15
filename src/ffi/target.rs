use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::SelectionKind;

use super::macros::flat_variants;
use super::macros::mirror_enum;

flat_variants!(PyCandidatePool {
    Hand => PyCandidatePoolHand as "CandidatePoolHand",
    Character => PyCandidatePoolCharacter as "CandidatePoolCharacter",
    Monsters => PyCandidatePoolMonsters as "CandidatePoolMonsters",
    Source => PyCandidatePoolSource as "CandidatePoolSource",
    Discover => PyCandidatePoolDiscover as "CandidatePoolDiscover",
    Deck => PyCandidatePoolDeck as "CandidatePoolDeck",
    EventPickCard => PyCandidatePoolEventPickCard as "CandidatePoolEventPickCard",
    EventPickPotion => PyCandidatePoolEventPickPotion as "CandidatePoolEventPickPotion",
    PileDraw => PyCandidatePoolPileDraw as "CandidatePoolPileDraw",
    PileDiscard => PyCandidatePoolPileDiscard as "CandidatePoolPileDiscard",
    PileExhaust => PyCandidatePoolPileExhaust as "CandidatePoolPileExhaust",
});

impl From<CandidatePool> for PyCandidatePool {
    fn from(pool: CandidatePool) -> Self {
        match pool {
            CandidatePool::Hand => Self::Hand(PyCandidatePoolHand),
            CandidatePool::Character => Self::Character(PyCandidatePoolCharacter),
            CandidatePool::Monsters => Self::Monsters(PyCandidatePoolMonsters),
            CandidatePool::Source => Self::Source(PyCandidatePoolSource),
            CandidatePool::Discover => Self::Discover(PyCandidatePoolDiscover),
            CandidatePool::Deck => Self::Deck(PyCandidatePoolDeck),
            CandidatePool::EventPickCard => Self::EventPickCard(PyCandidatePoolEventPickCard),
            CandidatePool::EventPickPotion => Self::EventPickPotion(PyCandidatePoolEventPickPotion),
            CandidatePool::PileDraw => Self::PileDraw(PyCandidatePoolPileDraw),
            CandidatePool::PileDiscard => Self::PileDiscard(PyCandidatePoolPileDiscard),
            CandidatePool::PileExhaust => Self::PileExhaust(PyCandidatePoolPileExhaust),
        }
    }
}

mirror_enum!(PyCandidateFilter from CandidateFilter, "CandidateFilter", skip_from_py_object, {
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
