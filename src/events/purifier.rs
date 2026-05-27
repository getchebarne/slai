use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
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

// Pray
const OPTION_PRAY: &[Effect] = &[
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

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_END_EFFECT];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Pray] Remove a card from your deck.",
        effects: OPTION_PRAY,
        gate: EventGate::HasPurgeableInDeck,
    },
    EventOption {
        label: "[Leave] Nothing happens.",
        effects: OPTION_LEAVE,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_PURIFIER: Entity = make_entity_event(EventName::Purifier, OPTIONS_ALL);
pub fn spawn_event_purifier() -> Entity {
    EVENT_PURIFIER
}
