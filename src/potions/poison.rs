use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::modifier::ModifierKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_POISON: Entity = make_entity_potion(
    PotionName::PoisonPotion,
    PotionRarity::Common,
    true,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 6,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
);
