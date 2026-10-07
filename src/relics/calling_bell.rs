use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain Curse of the Bell and a Common, Uncommon and Rare Relic; its Card roll is discarded
// See:
//    - `process_effect_relic_adopt.rs`
pub static CALLING_BELL: RelicTemplate = RelicTemplate {
    name: RelicName::CallingBell,
    tier: RelicTier::Boss,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[Effect {
        kind: EffectKind::RewardRollCards {
            bundles: 1,
            trigger: RewardRollTrigger::CallingBell,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_rest: &[],
    effects_counter: &[],
};
