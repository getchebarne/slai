use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static THREAD_AND_NEEDLE: RelicTemplate = RelicTemplate {
    name: RelicName::ThreadAndNeedle,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::PlatedArmor,
            stacks: 4,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
