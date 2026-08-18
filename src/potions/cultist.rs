use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_CULTIST: PotionTemplate = PotionTemplate {
    name: PotionName::CultistPotion,
    rarity: PotionRarity::Rare,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Ritual,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
