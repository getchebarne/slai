use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::modifier::ModifierKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_DUPLICATION: Entity = make_entity_potion(
    PotionName::DuplicateNextCardPlayPotion,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::DuplicateNextCardPlay,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
