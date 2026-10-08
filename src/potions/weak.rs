use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static WEAK: PotionTemplate = PotionTemplate {
    name: PotionName::Weak,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Weak,
            stacks: 3,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
};
// Doubled
pub static WEAK_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Weak,
            stacks: 6,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
    ..WEAK
};
