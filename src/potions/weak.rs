use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_WEAK: Entity = make_entity_potion(
    PotionName::WeakPotion,
    PotionRarity::Common,
    true,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 3,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
);
