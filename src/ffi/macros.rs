use pyo3::prelude::*;

use super::amount::PyDeltaSign;
use super::card::PyCardColor;
use super::card::PyCardKind;
use super::card::PyCardName;
use super::card::PyCardRarity;
use super::card::PyPlayRestriction;
use super::map::PyRoomKind;
use super::monster::PyIntentKind;
use super::monster::PyMonsterEncounter;
use super::monster::PyMonsterName;
use super::potion::PyPotionName;
use super::potion::PyPotionRarity;
use super::relic::PyRelicName;
use super::relic::PyRelicTier;
use super::target::PyCandidatePoolCardFilter;
use super::target::PyCandidatePoolMonstersFilter;

// Complex enums are exposed as one flat pyclass per variant. The Rust enum
// survives for composition; variant_union! gives it IntoPyObject dispatch plus
// a union OUTPUT_TYPE so generated stubs type fields as `VariantA | VariantB | ...`
macro_rules! variant_union {
    ($enum:ident { $($variant:ident => $cls:ident),+ $(,)? }) => {
        impl<'py> IntoPyObject<'py> for $enum {
            type Target = PyAny;
            type Output = Bound<'py, PyAny>;
            type Error = PyErr;
            const OUTPUT_TYPE: PyStaticExpr =
                type_hint_union!($(<$cls as PyTypeInfo>::TYPE_HINT),+);
            fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
                Ok(match self {
                    $( Self::$variant(v) => Bound::new(py, v)?.into_any(), )+
                })
            }
        }
    };
}

// pyo3's derived `hash` runs the discriminant through a hasher, so hash(enum) != hash(int)
// even though `eq_int` makes enum == int. That violates Python's eq/hash contract and makes
// these enums silently un-findable in int/IntEnum-keyed dicts. Hash by the raw discriminant
// so eq and hash agree.
macro_rules! impl_discriminant_hash {
    ($($ty:ty),+ $(,)?) => {
        $(
            #[pymethods]
            impl $ty {
                fn __hash__(&self) -> isize {
                    *self as isize
                }
            }
        )+
    };
}

impl_discriminant_hash!(
    PyCardKind,
    PyCardColor,
    PyCardRarity,
    PyPlayRestriction,
    PyDeltaSign,
    PyRoomKind,
    PyPotionName,
    PyPotionRarity,
    PyRelicName,
    PyCardName,
    PyMonsterName,
    PyMonsterEncounter,
    PyRelicTier,
    PyCandidatePoolMonstersFilter,
    PyCandidatePoolCardFilter,
    PyIntentKind,
);

pub(crate) use variant_union;
