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

// A15+: leave-gold range 20-50 → 35-75 (fixed at midpoint; no range-aware effect)

const GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: HealthDeltaSign::Loss,
            amount: HealthDeltaAmount::Flat(11),
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

static LEAVE_BASE: [Effect; 2] = leave(35);
static LEAVE_A15: [Effect; 2] = leave(55);

const fn options(leave_effects: &'static [Effect], leave_label: &'static str) -> [EventOption; 2] {
    [
        EventOption {
            label: "Gather Gold (lose 11 HP, +75 gold)",
            effects: GATHER,
            gate: EventGate::None,
        },
        EventOption {
            label: leave_label,
            effects: leave_effects,
            gate: EventGate::None,
        },
    ]
}

static OPTIONS_BASE: [EventOption; 2] = options(&LEAVE_BASE, "Leave (lose 35 gold)");
static OPTIONS_A15: [EventOption; 2] = options(&LEAVE_A15, "Leave (lose 55 gold)");

pub static GOOP_PUDDLE_BASE: Entity = make_entity_event(EventName::GoopPuddle, &OPTIONS_BASE);
pub static GOOP_PUDDLE_A15: Entity = make_entity_event(EventName::GoopPuddle, &OPTIONS_A15);

pub fn spawn_event_goop_puddle(ascension: u8) -> Entity {
    if ascension < 15 {
        GOOP_PUDDLE_BASE
    } else {
        GOOP_PUDDLE_A15
    }
}
