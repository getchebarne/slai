use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_card_play.rs`
pub static SHURIKEN: RelicTemplate = RelicTemplate {
    name: RelicName::Shuriken,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 3,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
