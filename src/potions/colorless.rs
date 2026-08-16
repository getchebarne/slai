use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::EFFECT_CARD_DISCOVER_PICK;
use crate::potions::make_entity_potion;
use crate::types::CardColor;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_COLORLESS: Entity = make_entity_potion(
    PotionName::ColorlessPotion,
    PotionRarity::Common,
    true,
    &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: None,
                color: CardColor::Colorless,
                exclude: &[],
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_CARD_DISCOVER_PICK,
    ],
);
