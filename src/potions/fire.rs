use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static FIRE: PotionTemplate = PotionTemplate {
    name: PotionName::Fire,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::DamageDeal {
            amount: 20,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
};
// Doubled
pub static FIRE_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::DamageDeal {
            amount: 40,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    ..FIRE
};
