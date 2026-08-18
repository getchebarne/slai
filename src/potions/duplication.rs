use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_DUPLICATION: PotionTemplate = PotionTemplate {
    name: PotionName::DuplicateNextCardPlayPotion,
    rarity: PotionRarity::Uncommon,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::DuplicateNextCardPlay,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
