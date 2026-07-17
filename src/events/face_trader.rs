use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::DeltaSign;
use crate::types::EventName;

// Touch: gold gain first, then health loss
const fn touch(gold: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(gold),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 10,
                },
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_TOUCH_BASE: [Effect; 3] = touch(75);
static OPTION_TOUCH_A15: [Effect; 3] = touch(50); // -25 gold gain

// Trade: gain random unowned face relic
const OPTION_TRADE: &[Effect] = &[
    Effect {
        kind: EffectKind::FaceTrade,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options
const fn options(touch_effects: &'static [Effect], touch_label: &'static str) -> [EventOption; 3] {
    [
        EventOption {
            label: touch_label,
            effects: touch_effects,
            gate: EventGate::None,
        },
        EventOption {
            label: "[Trade] Obtain a random face.",
            effects: OPTION_TRADE,
            gate: EventGate::None,
        },
        EventOption {
            label: "[Leave] Nothing happens.",
            effects: OPTION_LEAVE,
            gate: EventGate::None,
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 3] = options(
    &OPTION_TOUCH_BASE,
    "[Touch] Lose HP equal to 10% of Max HP. Gain 75 Gold.",
);
static OPTIONS_ALL_A15: [EventOption; 3] = options(
    &OPTION_TOUCH_A15,
    "[Touch] Lose HP equal to 10% of Max HP. Gain 50 Gold.",
);

// Export event
static EVENT_FACE_TRADER_BASE: Entity = make_entity_event(EventName::FaceTrader, &OPTIONS_ALL_BASE);
static EVENT_FACE_TRADER_A15: Entity = make_entity_event(EventName::FaceTrader, &OPTIONS_ALL_A15);
pub fn spawn_event_face_trader(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_FACE_TRADER_BASE
    } else {
        EVENT_FACE_TRADER_A15
    }
}
