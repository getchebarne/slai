use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::types::DeltaSign;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::EventName;

// Banana
const OPTION_BANANA: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: HealthDeltaAmount::Relative {
                numerator: 1,
                denominator: 3,
            },
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_END_EFFECT,
];

// Donut
const OPTION_DONUT: &[Effect] = &[
    Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: HealthDeltaAmount::Absolute(5),
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_END_EFFECT,
];

// Box
const OPTION_BOX: &[Effect] = &[
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Regret,
            upgraded: false,
        },
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

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Banana] Heal 1/3 of your max HP.",
        effects: OPTION_BANANA,
        gate: EventGate::None,
    },
    EventOption {
        label: "[Donut] Raise your max HP by 5.",
        effects: OPTION_DONUT,
        gate: EventGate::None,
    },
    EventOption {
        label: "[Box] Receive a Relic. Become Cursed - Regret.",
        effects: OPTION_BOX,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_BIG_FISH: Entity = make_entity_event(EventName::BigFish, OPTIONS_ALL);
pub fn spawn_event_big_fish() -> Entity {
    EVENT_BIG_FISH
}
