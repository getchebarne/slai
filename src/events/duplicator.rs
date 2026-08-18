use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventOptionTemplate;
use crate::events::OPTION_LEAVE;
use crate::events::make_event_option_template;

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
pub static OPTIONS: &[EventOptionTemplate] = &[
    make_event_option_template(
        "[Pray] Choose a card. Add a copy of it to your deck.",
        OPTION_PRAY,
    ),
    OPTION_LEAVE,
];
