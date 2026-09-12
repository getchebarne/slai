use crate::consts::ORRERY_BUNDLE_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On purchase, stages 5 Card bundles in the shop
// See:
//    - `process_effect_relic_adopt.rs`
pub static ORRERY: RelicTemplate = RelicTemplate {
    name: RelicName::Orrery,
    tier: RelicTier::Shop,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    // A 5-bundle Reward frame pushed over the shop; the stock resumes on exit
    effects_on_pickup: &[Effect {
        kind: EffectKind::RewardRollCards {
            bundles: ORRERY_BUNDLE_COUNT as u8,
            trigger: RewardRollTrigger::Orrery,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_on_rest: &[],
    effects_counter: &[],
};
