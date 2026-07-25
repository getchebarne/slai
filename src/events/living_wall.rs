use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_non_basic_non_curse;
use crate::events::deck_has_purgeable;
use crate::events::deck_has_upgradable;
use crate::game::GameState;

// Forget
const OPTION_FORGET: &[Effect] = &[
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

// Change
const OPTION_CHANGE: &[Effect] = &[
    Effect {
        kind: EffectKind::CardTransform,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::Transformable,
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
                filter: CandidatePoolCardFilter::Upgradeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

pub const OPTIONS: &[(&str, &[Effect])] = &[
    ("[Forget] Remove a card from your deck.", OPTION_FORGET),
    ("[Change] Transform a card in your deck.", OPTION_CHANGE),
    ("[Grow] Upgrade a card in your deck.", OPTION_GROW),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => deck_has_non_basic_non_curse(state),
        2 => deck_has_upgradable(state),
        _ => unreachable!("Living wall option out of range: {idx}"),
    }
}
