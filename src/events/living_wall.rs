use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Forget
const OPTION_FORGET: &[Effect] = &[
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
    EVENT_CONSUME_EFFECT,
];

// Change
const OPTION_CHANGE: &[Effect] = &[
    Effect {
        kind: EffectKind::CardTransform,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolDeckFilter::Transformable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Grow
const OPTION_GROW: &[Effect] = &[
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolDeckFilter::Upgradeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Forget] Remove a card from your deck.",
        effects: OPTION_FORGET,
        gate: EventGate::HasPurgeableInDeck,
    },
    EventOption {
        label: "[Change] Transform a card in your deck.",
        effects: OPTION_CHANGE,
        gate: EventGate::HasNonBasicNonCurseInDeck,
    },
    EventOption {
        label: "[Grow] Upgrade a card in your deck.",
        effects: OPTION_GROW,
        gate: EventGate::HasUpgradableInDeck,
    },
];

// Export event
static EVENT_LIVING_WALL: Entity = make_entity_event(EventName::LivingWall, OPTIONS_ALL);
pub fn spawn_event_living_wall() -> Entity {
    EVENT_LIVING_WALL
}
