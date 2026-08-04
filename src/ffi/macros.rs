use pyo3::prelude::*;

use super::amount::PyDeltaSign;
use super::card::PyCardColor;
use super::card::PyCardKind;
use super::card::PyCardName;
use super::card::PyCardPile;
use super::card::PyCardRarity;
use super::card::PyCostScope;
use super::card::PyPlayRestriction;
use super::map::PyRoomKind;
use super::monster::PyIntentKind;
use super::monster::PyMonsterEncounter;
use super::monster::PyMonsterName;
use super::potion::PyPotionName;
use super::potion::PyPotionRarity;
use super::relic::PyRelicName;
use super::relic::PyRelicTier;
use super::target::PyCandidateFilter;

// Complex enums are exposed as one flat pyclass per variant. flat_variants! takes one table
// per family and emits all three artifacts: the per-variant pyclasses, the Rust enum that
// survives for composition, and its IntoPyObject dispatch — whose union OUTPUT_TYPE makes
// generated stubs type fields as `VariantA | VariantB | ...`
//
// `hash` families derive Eq/Hash (their pyclasses are hashable); `plain` families hold PyCard
// and cannot. `get_all` follows from whether the variant carries fields.
macro_rules! flat_variants {
    (@cls hash $cls:ident, $name:literal { $($f:ident : $t:ty),+ $(,)? }) => {
        #[pyclass(
            skip_from_py_object,
            eq,
            hash,
            frozen,
            get_all,
            name = $name,
            module = "slai.slai"
        )]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $cls { $(pub $f: $t,)+ }
    };
    (@cls hash $cls:ident, $name:literal) => {
        #[pyclass(skip_from_py_object, eq, hash, frozen, name = $name, module = "slai.slai")]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $cls;
    };
    (@cls plain $cls:ident, $name:literal { $($f:ident : $t:ty),+ $(,)? }) => {
        #[pyclass(
            skip_from_py_object,
            frozen,
            get_all,
            name = $name,
            module = "slai.slai"
        )]
        #[derive(Debug, Clone)]
        pub struct $cls { $(pub $f: $t,)+ }
    };
    (@cls plain $cls:ident, $name:literal) => {
        #[pyclass(skip_from_py_object, frozen, name = $name, module = "slai.slai")]
        #[derive(Debug, Clone)]
        pub struct $cls;
    };

    (@enum hash $enum:ident { $($variant:ident => $cls:ident),+ }) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum $enum { $($variant($cls),)+ }
        flat_variants!(@into $enum { $($variant => $cls),+ });
    };
    (@enum plain $enum:ident { $($variant:ident => $cls:ident),+ }) => {
        #[derive(Debug, Clone)]
        pub enum $enum { $($variant($cls),)+ }
        flat_variants!(@into $enum { $($variant => $cls),+ });
    };

    (@into $enum:ident { $($variant:ident => $cls:ident),+ }) => {
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

    ($mode:tt $enum:ident {
        $($variant:ident => $cls:ident as $name:literal $({ $($f:ident : $t:ty),+ $(,)? })?),+ $(,)?
    }) => {
        $( flat_variants!(@cls $mode $cls, $name $({ $($f: $t),+ })?); )+
        flat_variants!(@enum $mode $enum { $($variant => $cls),+ });
    };
}

// 17 unit enums mirror an internal enum 1:1. mirror_enum! emits the pyclass declaration and
// the exhaustive From impl from one ident table; the match stays exhaustive, so a rename on
// either side is still a compile error
macro_rules! mirror_enum {
    ($py:ident from $internal:ident, $name:literal, $conv:tt, { $($v:ident),+ $(,)? }) => {
        #[pyclass($conv, eq, eq_int, frozen, name = $name, module = "slai.slai")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $py { $($v,)+ }

        impl From<$internal> for $py {
            fn from(v: $internal) -> Self {
                match v {
                    $($internal::$v => Self::$v,)+
                }
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
    PyCandidateFilter,
    PyCardPile,
    PyCostScope,
    PyIntentKind,
);

pub(crate) use flat_variants;
pub(crate) use mirror_enum;
