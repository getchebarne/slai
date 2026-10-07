use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static DUPLICATION: PotionTemplate = PotionTemplate {
    name: PotionName::Duplication,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::DuplicateNextCardPlay,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static DUPLICATION_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::DuplicateNextCardPlay,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..DUPLICATION
};
