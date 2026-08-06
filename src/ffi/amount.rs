use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::type_hint_union;
use pyo3::type_object::PyTypeInfo;

use crate::effect::Amount;
use crate::types::DeltaSign;

use super::macros::flat_variants;
use super::macros::mirror_enum;

mirror_enum!(PyDeltaSign from DeltaSign, "DeltaSign", skip_from_py_object, {
    Gain, Loss,
});

flat_variants!(PyAmount {
    Absolute => PyAmountAbsolute as "AmountAbsolute" { amount: u16 },
    Relative => PyAmountRelative as "AmountRelative" { numerator: u8, denominator: u8 },
    Range => PyAmountRange as "AmountRange" { min: u16, max: u16 },
    EventGoldAsk => PyAmountEventGoldAsk as "AmountEventGoldAsk",
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
