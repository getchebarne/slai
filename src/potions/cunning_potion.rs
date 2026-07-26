use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::CardName;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static CUNNING_POTION: Entity = make_entity_potion(
    PotionName::CunningPotion,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::CardAddToHand {
            card_name: CardName::Shiv,
            count: 3,
            upgraded: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
