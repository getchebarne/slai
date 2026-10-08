use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static DEXTERITY: PotionTemplate = PotionTemplate {
    name: PotionName::Dexterity,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Dexterity,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static DEXTERITY_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Dexterity,
            stacks: 4,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..DEXTERITY
};
