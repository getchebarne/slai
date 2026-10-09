use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static GHOST_IN_A_JAR: PotionTemplate = PotionTemplate {
    name: PotionName::GhostInAJar,
    rarity: PotionRarity::Rare,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Intangible,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static GHOST_IN_A_JAR_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Intangible,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..GHOST_IN_A_JAR
};
