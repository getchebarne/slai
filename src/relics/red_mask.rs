use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static RED_MASK: RelicTemplate = RelicTemplate {
    name: RelicName::RedMask,
    tier: RelicTier::Special,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
};
