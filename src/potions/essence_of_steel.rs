use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_ESSENCE_OF_STEEL: Entity = make_entity_potion(
    PotionName::EssenceOfSteel,
    PotionRarity::Uncommon,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::PlatedArmor,
            stacks: 4,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
