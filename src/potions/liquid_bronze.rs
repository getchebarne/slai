use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static LIQUID_BRONZE: PotionTemplate = PotionTemplate {
    name: PotionName::LiquidBronze,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Thorns,
            stacks: 3,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static LIQUID_BRONZE_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Thorns,
            stacks: 6,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..LIQUID_BRONZE
};
