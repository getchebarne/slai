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
    RelativeMinOne => PyAmountRelativeMinOne as "AmountRelativeMinOne" { numerator: u8, denominator: u8 },
    RelativeRounded => PyAmountRelativeRounded as "AmountRelativeRounded" { numerator: u8, denominator: u8 },
    RelativeCeil => PyAmountRelativeCeil as "AmountRelativeCeil" { numerator: u8, denominator: u8 },
    Range => PyAmountRange as "AmountRange" { min: u16, max: u16 },
});

impl From<Amount> for PyAmount {
    fn from(amount: Amount) -> Self {
        match amount {
            Amount::Absolute(amount) => Self::Absolute(PyAmountAbsolute { amount }),
            Amount::Relative {
                numerator,
                denominator,
            } => Self::Relative(PyAmountRelative {
                numerator,
                denominator,
            }),
            Amount::RelativeMinOne {
                numerator,
                denominator,
            } => Self::RelativeMinOne(PyAmountRelativeMinOne {
                numerator,
                denominator,
            }),
            Amount::RelativeRounded {
                numerator,
                denominator,
            } => Self::RelativeRounded(PyAmountRelativeRounded {
                numerator,
                denominator,
            }),
            Amount::RelativeCeil {
                numerator,
                denominator,
            } => Self::RelativeCeil(PyAmountRelativeCeil {
                numerator,
                denominator,
            }),
            Amount::Range { min, max } => Self::Range(PyAmountRange { min, max }),
        }
    }
}

// Health / MaxHealth deltas never carry Range; the narrower union keeps the stub truthful
flat_variants!(@enum PyAmountScalar {
    Absolute => PyAmountAbsolute,
    Relative => PyAmountRelative,
    RelativeMinOne => PyAmountRelativeMinOne,
    RelativeRounded => PyAmountRelativeRounded,
    RelativeCeil => PyAmountRelativeCeil
});

impl From<Amount> for PyAmountScalar {
    fn from(amount: Amount) -> Self {
        match PyAmount::from(amount) {
            PyAmount::Absolute(v) => Self::Absolute(v),
            PyAmount::Relative(v) => Self::Relative(v),
            PyAmount::RelativeMinOne(v) => Self::RelativeMinOne(v),
            PyAmount::RelativeRounded(v) => Self::RelativeRounded(v),
            PyAmount::RelativeCeil(v) => Self::RelativeCeil(v),
            PyAmount::Range(_) => unreachable!("health deltas never carry Amount::Range"),
        }
    }
}

// Gold deltas only truncate their fractions; the narrower union keeps the stub truthful
flat_variants!(@enum PyAmountGold {
    Absolute => PyAmountAbsolute,
    Relative => PyAmountRelative,
    Range => PyAmountRange
});

impl From<Amount> for PyAmountGold {
    fn from(amount: Amount) -> Self {
        match PyAmount::from(amount) {
            PyAmount::Absolute(v) => Self::Absolute(v),
            PyAmount::Relative(v) => Self::Relative(v),
            PyAmount::Range(v) => Self::Range(v),
            PyAmount::RelativeMinOne(_)
            | PyAmount::RelativeRounded(_)
            | PyAmount::RelativeCeil(_) => {
                unreachable!("gold deltas carry only Absolute, Relative or Range")
            }
        }
    }
}
