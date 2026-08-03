use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_CUNNING: Entity = make_entity_potion(
    PotionName::CunningPotion,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count: 3,
            upgraded: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
