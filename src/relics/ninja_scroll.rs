use crate::effect::{Effect, EffectKind, Target};
use crate::entity::{Entity, make_entity_relic};
use crate::types::{CardName, RelicName, RelicTier};

pub static NINJA_SCROLL: Entity = make_entity_relic(
    RelicName::NinjaScroll,
    RelicTier::Uncommon,
    0,
    &[Effect {
        kind: EffectKind::CardAddToHand {
            card_name: CardName::Shiv,
            count: 3,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
