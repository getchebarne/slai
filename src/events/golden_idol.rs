use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::EventName;
use crate::types::RelicName;

// Take; TODO: swap placeholder Circlet for GoldenIdol once that relic exists
const OPTION_TAKE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::Circlet,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::EventAdvanceState { delta: 1 },
        id_source: None,
        target: Target::Direct(None),
    },
];
// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_END_EFFECT];
// Outrun
const OPTION_OUTRUN: &[Effect] = &[
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Injury,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];
// Smash
const OPTION_SMASH: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: HealthDeltaSign::Loss,
            amount: HealthDeltaAmount::Relative {
                numerator: 1,
                denominator: 4,
            },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];
// Hide
const OPTION_HIDE: &[Effect] = &[
    Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: HealthDeltaSign::Loss,
            amount: HealthDeltaAmount::Relative {
                numerator: 8,
                denominator: 100,
            },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "Take",
        effects: OPTION_TAKE,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "Leave",
        effects: OPTION_LEAVE,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "Outrun (+Injury curse)",
        effects: OPTION_OUTRUN,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "Smash (lose 25% max HP)",
        effects: OPTION_SMASH,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "Hide (lose 8% max HP cap)",
        effects: OPTION_HIDE,
        gate: EventGate::EventStateEq(1),
    },
];

// Export event
static EVENT_GOLDEN_IDOL: Entity = make_entity_event(EventName::GoldenIdol, OPTIONS_ALL);
pub fn spawn_event_golden_idol(_ascension: u8) -> Entity {
    EVENT_GOLDEN_IDOL
}
