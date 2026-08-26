use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::Amount;
use crate::types::DeltaSign;

use super::macros::flat_variants;
use super::macros::mirror_enum;

mirror_enum!(PyDeltaSign from DeltaSign, "DeltaSign", {
    Gain, Loss,
});

flat_variants!(PyAmount {
    Absolute => PyAmountAbsolute as "AmountAbsolute" { amount: u16 },
    Relative => PyAmountRelative as "AmountRelative" { numerator: u8, denominator: u8 },
    Range => PyAmountRange as "AmountRange" { min: u16, max: u16 },
});

impl From<Amount> for PyAmount {
    fn from(amount: Amount) -> Self {
        match amount {
            Amount::Absolute(amount) => Self::Absolute(PyAmountAbsolute { amount }),
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
        }
    }
}

// Health / MaxHealth deltas never carry Range; the narrower union keeps the stub truthful
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PyAmountScalar {
    Absolute(PyAmountAbsolute),
    Relative(PyAmountRelative),
}

impl<'py> IntoPyObject<'py> for PyAmountScalar {
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;
    type Error = PyErr;
    const OUTPUT_TYPE: PyStaticExpr = type_hint_union!(
        <PyAmountAbsolute as PyTypeInfo>::TYPE_HINT,
        <PyAmountRelative as PyTypeInfo>::TYPE_HINT
    );
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        Ok(match self {
            Self::Absolute(v) => Bound::new(py, v)?.into_any(),
            Self::Relative(v) => Bound::new(py, v)?.into_any(),
        })
    }
}

impl From<Amount> for PyAmountScalar {
    fn from(amount: Amount) -> Self {
        match PyAmount::from(amount) {
            PyAmount::Absolute(absolute) => Self::Absolute(absolute),
            PyAmount::Relative(relative) => Self::Relative(relative),
            PyAmount::Range(_) => unreachable!("health deltas never carry Amount::Range"),
        }
    }
}
