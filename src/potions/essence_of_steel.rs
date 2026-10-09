use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static ESSENCE_OF_STEEL: PotionTemplate = PotionTemplate {
    name: PotionName::EssenceOfSteel,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::PlatedArmor,
            stacks: 4,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static ESSENCE_OF_STEEL_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::PlatedArmor,
            stacks: 8,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..ESSENCE_OF_STEEL
};
