use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::EventName;
use crate::types::RelicName;

// TODO: swap placeholder Circlet for GoldenIdol once that relic exists
const TAKE: &[Effect] = &[
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

const LEAVE_SCREEN_0: &[Effect] = &[EVENT_END_EFFECT];

const OUTRUN: &[Effect] = &[
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

const SMASH: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthLossPct { numer: 1, denom: 4 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const HIDE: &[Effect] = &[
    Effect {
        kind: EffectKind::MaxHealthLossPct { numer: 8, denom: 100 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Take",
        effects: TAKE,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "Leave",
        effects: LEAVE_SCREEN_0,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "Outrun (+Injury curse)",
        effects: OUTRUN,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "Smash (lose 25% max HP)",
        effects: SMASH,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "Hide (lose 8% max HP cap)",
        effects: HIDE,
        gate: EventGate::EventStateEq(1),
    },
];

pub static GOLDEN_IDOL_EVENT: Entity = make_entity_event(EventName::GoldenIdolEvent, OPTIONS);
