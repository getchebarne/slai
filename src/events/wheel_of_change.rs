use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;

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
pub const LABELS: &[&str] =
    &["[Spin] Gold, a relic, a full heal, a Decay, a card removal, or HP loss."];

pub fn push_option_effects(buf: &mut Vec<Effect>, idx: usize) {
    buf.extend_from_slice(match idx {
        0 => OPTION_SPIN,
        _ => unreachable!("wheel of change option out of range: {idx}"),
    });
}
