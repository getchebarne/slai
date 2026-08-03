use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;

// Drink: purge every removable curse at once
const OPTION_DRINK: &[Effect] = &[
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::PurgeableCurse,
            selection_kind: SelectionKind::All,
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
// The event only spawns with a removable curse in the deck
pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Drink] Remove all Curses from your deck.", OPTION_DRINK),
    OPTION_LEAVE,
];
