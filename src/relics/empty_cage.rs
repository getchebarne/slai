use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, remove 2 Cards from the deck
// See:
//    - `process_effect_relic_adopt.rs`
pub static EMPTY_CAGE: RelicTemplate = RelicTemplate {
    name: RelicName::EmptyCage,
    tier: RelicTier::Boss,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    // Remove 2 Cards from the deck
    effects_on_pickup: &[
        Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck,
                filter: CandidateFilter::Purgeable,
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
        Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck,
                filter: CandidateFilter::Purgeable,
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
    ],
    effects_on_rest: &[],
    effects_counter: &[],
};
