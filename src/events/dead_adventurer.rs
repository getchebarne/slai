use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Search: escalating elite-return chance; the AdventurerSearch processor grants
// the pre-rolled loot, advances event_state, and consumes after the third find
const OPTION_SEARCH: &[Effect] = &[Effect {
    kind: EffectKind::AdventurerSearch,
    id_source: None,
    target: Target::Direct(None),
}];

// Escape
const OPTION_ESCAPE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Search] Find loot; the telegraphed elite may return.",
        effects: OPTION_SEARCH,
        gate: EventGate::None,
    },
    EventOption {
        label: "[Escape] Leave with what you found.",
        effects: OPTION_ESCAPE,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_DEAD_ADVENTURER: Entity = make_entity_event(EventName::DeadAdventurer, OPTIONS_ALL);
pub fn spawn_event_dead_adventurer() -> Entity {
    EVENT_DEAD_ADVENTURER
}
