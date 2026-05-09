use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::types::{RelicName, RelicTier};

pub static SNAKE_RING: Entity = make_entity_relic(
    RelicName::SnakeRing,
    RelicTier::Starter,
    0,
    &[Effect {
        kind: EffectKind::CardDraw { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    }],
);
