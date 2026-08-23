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
    effects: &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::LoseStrength,
                stacks: 5,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
};
