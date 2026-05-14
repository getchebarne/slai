use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::CardName;
use crate::types::RelicName;
use crate::types::RelicTier;

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
