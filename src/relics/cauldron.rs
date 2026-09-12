use crate::consts::CAULDRON_POTION_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On purchase, brews 5 Potions staged as a reward over the shop
// See:
//    - `process_effect_relic_adopt.rs`
pub static CAULDRON: RelicTemplate = RelicTemplate {
    name: RelicName::Cauldron,
    tier: RelicTier::Shop,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    // Brews 5 Potions, staged as a Reward frame over the shop
    effects_on_pickup: &[Effect {
        kind: EffectKind::RewardRollPotions {
            count: CAULDRON_POTION_COUNT as u8,
            uniform: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_on_rest: &[],
    effects_counter: &[],
};
