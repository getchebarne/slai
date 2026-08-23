use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_purgeable;
use crate::events::opt;
use crate::game::GameState;

// Simplicity: every un-upgraded Strike and Defend upgrades
const OPTION_SIMPLICITY: &[Effect] = &[
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::StarterUpgradeable,
            selection_kind: SelectionKind::All,
        },
    },
    EVENT_CONSUME_EFFECT,
];

const OPTION_ELEGANCE: &[Effect] = &[EFFECT_DECK_PURGE_PICK_1, EVENT_CONSUME_EFFECT];

pub static OPTIONS: &[&[Effect]] = &[opt(OPTION_ELEGANCE), opt(OPTION_SIMPLICITY)];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        _ => true,
    }
}
