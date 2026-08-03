use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_LIQUID_BRONZE: Entity = make_entity_potion(
    PotionName::LiquidBronze,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Thorns,
            stacks: 3,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
