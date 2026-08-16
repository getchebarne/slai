use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::DeltaSign;
use crate::types::PotionName;
use crate::types::PotionRarity;

// The MaxHealthDelta handler heals too, matching StS `increaseMaxHp(5, true)`
pub static POTION_FRUIT_JUICE: Entity = make_entity_potion(
    PotionName::FruitJuice,
    PotionRarity::Rare,
    false,
    &[Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(5),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
