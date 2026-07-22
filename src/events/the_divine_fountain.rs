use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;

// Drink: purge every removable curse at once
const OPTION_DRINK: &[Effect] = &[
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::PurgeableCurse,
            },
            selection_kind: SelectionKind::All,
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// The event only spawns with a removable curse in the deck
pub const OPTIONS: &[(&str, &[Effect])] = &[
    ("[Drink] Remove all Curses from your deck.", OPTION_DRINK),
    ("[Leave] Nothing happens.", OPTION_LEAVE),
];
