use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static STEROID: PotionTemplate = PotionTemplate {
    name: PotionName::Steroid,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
    effects: &[
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::LoseStrength,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
};
// Doubled
pub static STEROID_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: 10,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::LoseStrength,
                stacks: 10,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    ..STEROID
};
