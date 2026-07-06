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

// Gather
const OPTION_GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(11),
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(75),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const fn leave(min: u16, max: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Range { min, max },
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_LEAVE_BASE: [Effect; 2] = leave(20, 50);
static OPTION_LEAVE_A15: [Effect; 2] = leave(35, 75);

// All options
const fn options(leave_effects: &'static [Effect], leave_label: &'static str) -> [EventOption; 2] {
    [
        EventOption {
            label: "[Gather Gold] Gain 75 Gold. Lose 11 HP.",
            effects: OPTION_GATHER,
            gate: EventGate::None,
        },
        EventOption {
            label: leave_label,
            effects: leave_effects,
            gate: EventGate::None,
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 2] =
    options(&OPTION_LEAVE_BASE, "[Leave It] Lose 20-50 Gold.");
static OPTIONS_ALL_A15: [EventOption; 2] =
    options(&OPTION_LEAVE_A15, "[Leave It] Lose 35-75 Gold.");

// Export event
static EVENT_WORLD_OF_GOOP_BASE: Entity =
    make_entity_event(EventName::WorldOfGoop, &OPTIONS_ALL_BASE);
static EVENT_WORLD_OF_GOOP_A15: Entity =
    make_entity_event(EventName::WorldOfGoop, &OPTIONS_ALL_A15);
pub fn spawn_event_world_of_goop(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_WORLD_OF_GOOP_BASE
    } else {
        EVENT_WORLD_OF_GOOP_A15
    }
}
