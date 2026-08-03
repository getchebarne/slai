use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::modifier::ModifierKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_REGENERATION: Entity = make_entity_potion(
    PotionName::RegenerationPotion,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Regeneration,
            stacks: 5,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
