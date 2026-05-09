use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

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
