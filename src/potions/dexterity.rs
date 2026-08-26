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
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Dexterity,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
