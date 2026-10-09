use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, upgrade 2 random Skills
// See:
//    - `process_effect_relic_adopt.rs`
pub static WAR_PAINT: RelicTemplate = RelicTemplate {
    name: RelicName::WarPaint,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filters: &[CandidateFilter::Upgradeable, CandidateFilter::KindSkill],
            selection_kind: SelectionKind::Random { count: 2 },
        },
    }],
    effects_rest: &[],
    effects_counter: &[],
};
