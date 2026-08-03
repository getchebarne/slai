use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static ANCHOR: Entity = make_entity_relic(
    RelicName::Anchor,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 10 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
