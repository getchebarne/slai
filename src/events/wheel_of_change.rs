use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
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
pub static OPTIONS: &[Entity] = &[make_entity_event_option(
    "[Spin] Gold, a relic, a full heal, a Decay, a card removal, or HP loss.",
    OPTION_SPIN,
)];
