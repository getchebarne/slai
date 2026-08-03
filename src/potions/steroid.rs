use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_STEROID: Entity = make_entity_potion(
    PotionName::SteroidPotion,
    PotionRarity::Common,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::LoseStrength,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
);
