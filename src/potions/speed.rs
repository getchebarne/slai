use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::modifier::ModifierKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_SPEED: Entity = make_entity_potion(
    PotionName::SpeedPotion,
    PotionRarity::Common,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Dexterity,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::LoseDexterity,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
);
