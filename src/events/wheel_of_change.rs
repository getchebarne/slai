use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;

// Spin
const OPTION_SPIN: &[Effect] = &[
    Effect {
        kind: EffectKind::WheelSpin,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Spin is mandatory, there's no "Leave" option
pub static OPTIONS: &[&[Effect]] = &[opt(OPTION_SPIN)];
