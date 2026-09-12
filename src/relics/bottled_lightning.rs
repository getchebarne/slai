use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, bottle a Skill; it starts every combat in the opening hand
// See:
//    - `process_effect_relic_adopt.rs`
//    - `process_effect_card_bottle.rs`
//    - `process_effect_combat_start.rs`
pub static BOTTLED_LIGHTNING: RelicTemplate = RelicTemplate {
    name: RelicName::BottledLightning,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    // Bottle a deck Card of the kind; an empty pool auto-resolves to no pick
    effects_on_pickup: &[Effect {
        kind: EffectKind::CardBottle,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::KindSkill,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    }],
    effects_on_rest: &[],
    effects_counter: &[],
};
