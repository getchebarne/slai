use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_purgeable;
use crate::game::GameState;

// Pray
const OPTION_PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::Purgeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

pub const OPTIONS: &[(&str, &[Effect])] = &[
    ("[Pray] Remove a card from your deck.", OPTION_PRAY),
    ("[Leave] Nothing happens.", OPTION_LEAVE),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => true,
        _ => unreachable!("Purifier option out of range: {idx}"),
    }
}
