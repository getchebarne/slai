use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static BAG_OF_PREPARATION: Entity = make_entity_relic(
    RelicName::BagOfPreparation,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::CardDraw { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    }],
);
