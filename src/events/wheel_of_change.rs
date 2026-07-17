use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

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
const OPTIONS_ALL: &[EventOption] = &[EventOption {
    label: "[Spin] Gold, a relic, a full heal, a Decay, a card removal, or HP loss.",
    effects: OPTION_SPIN,
    gate: EventGate::None,
}];

// Export event
static EVENT_WHEEL_OF_CHANGE: Entity = make_entity_event(EventName::WheelOfChange, OPTIONS_ALL);
pub fn spawn_event_wheel_of_change() -> Entity {
    EVENT_WHEEL_OF_CHANGE
}
