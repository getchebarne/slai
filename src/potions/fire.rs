use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_FIRE: PotionTemplate = PotionTemplate {
    name: PotionName::FirePotion,
    rarity: PotionRarity::Common,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 20,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
};
