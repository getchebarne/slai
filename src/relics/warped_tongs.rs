use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

pub static WARPED_TONGS: RelicTemplate = RelicTemplate {
    name: RelicName::WarpedTongs,
    tier: RelicTier::Special,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Hand,
            filter: CandidateFilter::Upgradeable,
            selection_kind: SelectionKind::Random { count: 1 },
        },
    }],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[],
};
