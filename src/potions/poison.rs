use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTER_PICKED;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_POISON: PotionTemplate = PotionTemplate {
    name: PotionName::PoisonPotion,
    rarity: PotionRarity::Common,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Poison,
            stacks: 6,
        },
        id_source: None,
        target: TARGET_MONSTER_PICKED,
    }],
};
