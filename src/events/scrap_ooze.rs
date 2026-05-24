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

// Single-shot; StS escalates dmg + chance per Reach until hit

const HIT_BRANCH: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantRandom { tier: None },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const MISS_BRANCH: &[Effect] = &[EVENT_END_EFFECT];

const REACH: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthLoss { amount: 5 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::RollD100Branch {
            chance: 35,
            on_lt: HIT_BRANCH,
            on_ge: MISS_BRANCH,
        },
        id_source: None,
        target: Target::Direct(None),
    },
];

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Reach in (lose 5 HP, 35% chance of relic)",
        effects: REACH,
        gate: EventGate::None,
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static SCRAP_OOZE: Entity = make_entity_event(EventName::ScrapOoze, OPTIONS);
