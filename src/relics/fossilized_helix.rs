use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Start each combat with 1 Buffer (prevent the next HP loss)
// See:
//    - `process_effect_combat_start.rs`
pub static FOSSILIZED_HELIX: RelicTemplate = RelicTemplate {
    name: RelicName::FossilizedHelix,
    tier: RelicTier::Rare,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Buffer,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
