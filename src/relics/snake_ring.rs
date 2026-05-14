use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

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
