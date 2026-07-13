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

// Offer: pick a card to purge; the reward keys on its rarity (see BonfireOffer).
// An empty purgeable pool auto-resolves to nothing, matching the patched source
const OPTION_OFFER: &[Effect] = &[
    Effect {
        kind: EffectKind::BonfireOffer,
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

const OPTIONS_ALL: &[EventOption] = &[EventOption {
    label: "[Offer] Remove a card; its rarity decides the spirits' blessing.",
    effects: OPTION_OFFER,
    gate: EventGate::None,
}];

// Export event
static EVENT_BONFIRE_SPIRITS: Entity = make_entity_event(EventName::BonfireSpirits, OPTIONS_ALL);
pub fn spawn_event_bonfire_spirits() -> Entity {
    EVENT_BONFIRE_SPIRITS
}
