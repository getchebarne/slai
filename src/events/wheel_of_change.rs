use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventOptionTemplate;
use crate::events::make_event_option_template;

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
pub static OPTIONS: &[EventOptionTemplate] = &[make_event_option_template(
    "[Spin] Gold, a relic, a full heal, a Decay, a card removal, or HP loss.",
    OPTION_SPIN,
)];
