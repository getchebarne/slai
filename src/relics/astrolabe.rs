use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, choose 3 Cards to transform; the results are upgraded
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_astrolabe_transform.rs`
pub static ASTROLABE: RelicTemplate = RelicTemplate {
    name: RelicName::Astrolabe,
    tier: RelicTier::Boss,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    // Choose 3 Cards to transform; the results are upgraded
    effects_on_pickup: &[Effect {
        kind: EffectKind::CardTransform { upgraded: true },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Transformable,
            selection_kind: SelectionKind::Input { count: 3 },
        },
    }],
    effects_on_rest: &[],
    effects_counter: &[],
};
