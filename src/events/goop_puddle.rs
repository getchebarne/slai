use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

const GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthLoss { amount: 11 },
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

// StS uses rand(20,50); fixed at midpoint pending range-aware effect kinds
const LEAVE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 35 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Gather Gold (lose 11 HP, +75 gold)",
        effects: GATHER,
        gate: EventGate::None,
    },
    EventOption {
        label: "Leave (lose 35 gold)",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static GOOP_PUDDLE: Entity = make_entity_event(EventName::GoopPuddle, OPTIONS);
