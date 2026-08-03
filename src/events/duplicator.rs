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

// Pray
const OPTION_PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::CardDuplicate,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
pub static OPTIONS: &[Entity] = &[
    make_entity_event_option(
        "[Pray] Choose a card. Add a copy of it to your deck.",
        OPTION_PRAY,
    ),
    OPTION_LEAVE,
];
