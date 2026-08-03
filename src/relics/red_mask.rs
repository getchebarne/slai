use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static RED_MASK: Entity = make_entity_relic(
    RelicName::RedMask,
    RelicTier::Special,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
);
