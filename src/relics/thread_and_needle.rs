use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::modifier::ModifierKind;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static THREAD_AND_NEEDLE: Entity = make_entity_relic(
    RelicName::ThreadAndNeedle,
    RelicTier::Rare,
    0,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::PlatedArmor,
            stacks: 4,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
