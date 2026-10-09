use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::Amount;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::types::DeltaSign;

use super::macros::flat_variants;
use super::macros::mirror_enum;

mirror_enum!(PyDeltaSign from DeltaSign, "DeltaSign", {
    Gain, Loss,
});

mirror_enum!(PyReadAt from ReadAt, "ReadAt", {
    Now, EventOpen,
});

mirror_enum!(PyRounding from Rounding, "Rounding", {
    Truncate, TruncateMinOne, HalfUp, Ceil,
});

flat_variants!(PyAmount {
    Absolute => PyAmountAbsolute as "AmountAbsolute" { amount: u16 },
    Relative => PyAmountRelative as "AmountRelative" { numerator: u8, denominator: u8, rounding: PyRounding, read_at: PyReadAt },
    Range => PyAmountRange as "AmountRange" { min: u16, max: u16 },
});

impl From<Amount> for PyAmount {
    fn from(amount: Amount) -> Self {
        match amount {
            Amount::Absolute(amount) => Self::Absolute(PyAmountAbsolute { amount }),
            Amount::Relative {
                numerator,
                denominator,
                rounding,
                read_at,
            } => Self::Relative(PyAmountRelative {
                numerator,
                denominator,
                rounding: rounding.into(),
                read_at: read_at.into(),
            }),
            Amount::Range { min, max } => Self::Range(PyAmountRange { min, max }),
        }
    }
}

// Health / MaxHealth deltas never carry Range; the narrower union keeps the stub truthful
flat_variants!(@enum PyAmountScalar {
    Absolute => PyAmountAbsolute,
    Relative => PyAmountRelative
});

impl From<Amount> for PyAmountScalar {
    fn from(amount: Amount) -> Self {
        match PyAmount::from(amount) {
            PyAmount::Absolute(v) => Self::Absolute(v),
            PyAmount::Relative(v) => Self::Relative(v),
            PyAmount::Range(_) => unreachable!("health deltas never carry Amount::Range"),
        }
    }
}
