use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, choose a deck Card and obtain a copy of it
// See:
//    - `process_effect_relic_adopt.rs`
pub static DOLLYS_MIRROR: RelicTemplate = RelicTemplate {
    name: RelicName::DollysMirror,
    tier: RelicTier::Shop,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[Effect {
        kind: EffectKind::CardDuplicate,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }],
    effects_rest: &[],
    effects_counter: &[],
};
