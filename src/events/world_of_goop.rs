use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Gather
const OPTION_GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: HealthDeltaSign::Loss,
            amount: HealthDeltaAmount::Absolute(11),
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::GoldGain { amount: 75 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];
// Leave; StS rolls 20–50 (base) / 35–75 (A15+), fixed at midpoint pending range-aware effects
const fn leave(gold_loss: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldLoss { amount: gold_loss },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_END_EFFECT,
    ]
}
static OPTION_LEAVE_BASE: [Effect; 2] = leave(35);
static OPTION_LEAVE_A15: [Effect; 2] = leave(55); // +20 gold cost

// All options
const fn options(leave_effects: &'static [Effect], leave_label: &'static str) -> [EventOption; 2] {
    [
        EventOption {
            label: "Gather Gold (lose 11 HP, +75 gold)",
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
static OPTIONS_ALL_BASE: [EventOption; 2] = options(&OPTION_LEAVE_BASE, "Leave (lose 35 gold)");
static OPTIONS_ALL_A15: [EventOption; 2] = options(&OPTION_LEAVE_A15, "Leave (lose 55 gold)");

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
