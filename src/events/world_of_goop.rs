use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;
use crate::types::DeltaSign;

// Gather
const OPTION_GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(11),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(75),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const fn leave(min: u16, max: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Range { min, max },
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_LEAVE_BASE: [Effect; 2] = leave(20, 50);
const OPTION_LEAVE_A15: [Effect; 2] = leave(35, 75);

static OPTIONS_BASE: &[&[Effect]] = &[opt(OPTION_GATHER), opt(&OPTION_LEAVE_BASE)];
static OPTIONS_A15: &[&[Effect]] = &[opt(OPTION_GATHER), opt(&OPTION_LEAVE_A15)];

pub fn options(ascension: u8) -> &'static [&'static [Effect]] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
