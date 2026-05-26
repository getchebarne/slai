use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
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

// Pray
const OPTION_PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: HealthDeltaSign::Loss,
            amount: HealthDeltaAmount::Absolute(7),
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolDeckFilter::Purgeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_END_EFFECT,
];
// Attack; StS rolls 50–80 gold, fixed at midpoint pending range-aware effects
const OPTION_ATTACK: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldGain { amount: 65 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];
// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_END_EFFECT];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "Pray (lose 7 HP, remove a card)",
        effects: OPTION_PRAY,
        gate: EventGate::HasPurgeableInDeck,
    },
    EventOption {
        label: "Attack (+65 gold)",
        effects: OPTION_ATTACK,
        gate: EventGate::HasDamageCardInDeck { min_base: 10 },
    },
    EventOption {
        label: "Leave",
        effects: OPTION_LEAVE,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_WING_STATUE: Entity = make_entity_event(EventName::WingStatue, OPTIONS_ALL);
pub fn spawn_event_wing_statue(_ascension: u8) -> Entity {
    EVENT_WING_STATUE
}
