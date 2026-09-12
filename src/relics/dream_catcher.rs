use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Resting at a rest site also offers a Card reward
// See:
//    - `action.rs`
//    - `process_effect_reward_roll_cards.rs`
pub static DREAM_CATCHER: RelicTemplate = RelicTemplate {
    name: RelicName::DreamCatcher,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    // Resting also offers a Card reward (Rest only, not Smith)
    effects_on_rest: &[Effect {
        kind: EffectKind::RewardRollCards {
            bundles: 1,
            trigger: RewardRollTrigger::DreamCatcher,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_counter: &[],
};
