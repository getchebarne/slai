use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static ODDLY_SMOOTH_STONE: RelicTemplate = RelicTemplate {
    name: RelicName::OddlySmoothStone,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Dexterity,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
