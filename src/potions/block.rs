use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_BLOCK: Entity = make_entity_potion(
    PotionName::BlockPotion,
    PotionRarity::Common,
    false,
    true,
    &[Effect {
        kind: EffectKind::BlockGain { amount: 12 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
