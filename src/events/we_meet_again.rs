use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Give gold; StS also offers a rolled potion or card (omitted)
const OPTION_GIVE_GOLD: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 100 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::RelicGrantRandom,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];
// Refuse
const OPTION_REFUSE: &[Effect] = &[EVENT_END_EFFECT];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "Give 100 gold (gain random relic)",
        effects: OPTION_GIVE_GOLD,
        gate: EventGate::GoldAtLeast(100),
    },
    EventOption {
        label: "Refuse",
        effects: OPTION_REFUSE,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_WE_MEET_AGAIN: Entity = make_entity_event(EventName::WeMeetAgain, OPTIONS_ALL);
pub fn spawn_event_we_meet_again(_ascension: u8) -> Entity {
    EVENT_WE_MEET_AGAIN
}
