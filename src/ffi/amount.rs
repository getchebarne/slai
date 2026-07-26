use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::Amount;
use crate::types::DeltaSign;

use super::macros::variant_union;

#[pyclass(
    skip_from_py_object,
    eq,
    eq_int,
    frozen,
    name = "DeltaSign",
    module = "slai.slai"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PyDeltaSign {
    Gain,
    Loss,
}

impl From<DeltaSign> for PyDeltaSign {
    fn from(sign: DeltaSign) -> Self {
        match sign {
            DeltaSign::Gain => Self::Gain,
            DeltaSign::Loss => Self::Loss,
        }
    }
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "AmountAbsolute",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountAbsolute {
    pub amount: u16,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "AmountRelative",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountRelative {
    pub numerator: u8,
    pub denominator: u8,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    get_all,
    name = "AmountRange",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountRange {
    pub min: u16,
    pub max: u16,
}

#[pyclass(
    skip_from_py_object,
    eq,
    hash,
    frozen,
    name = "AmountEventGoldAsk",
    module = "slai.slai"
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyAmountEventGoldAsk;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyAmount {
    Absolute(PyAmountAbsolute),
    Relative(PyAmountRelative),
    Range(PyAmountRange),
    EventGoldAsk(PyAmountEventGoldAsk),
}

variant_union!(PyAmount {
    Absolute => PyAmountAbsolute,
    Relative => PyAmountRelative,
    Range => PyAmountRange,
    EventGoldAsk => PyAmountEventGoldAsk,
});

impl From<Amount> for PyAmount {
    fn from(amount: Amount) -> Self {
        match amount {
            Amount::Absolute(amount) => Self::Absolute(PyAmountAbsolute { amount }),
            // Rounding mode is engine-internal; the view keeps one Relative shape
            Amount::Relative {
                numerator,
                denominator,
            }
            | Amount::RelativeRounded {
                numerator,
                denominator,
            }
            | Amount::RelativeCeil {
                numerator,
                denominator,
            } => Self::Relative(PyAmountRelative {
                numerator,
                denominator,
            }),
            Amount::Range { min, max } => Self::Range(PyAmountRange { min, max }),
            Amount::EventGoldAsk => Self::EventGoldAsk(PyAmountEventGoldAsk),
        }
    }
}
