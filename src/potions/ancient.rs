use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static ANCIENT: PotionTemplate = PotionTemplate {
    name: PotionName::Ancient,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Artifact,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static ANCIENT_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Artifact,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..ANCIENT
};
