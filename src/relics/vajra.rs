use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static VAJRA: Entity = make_entity_relic(
    RelicName::Vajra,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
