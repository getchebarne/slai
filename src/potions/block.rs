use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_BLOCK: PotionTemplate = PotionTemplate {
    name: PotionName::BlockPotion,
    rarity: PotionRarity::Common,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 12 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
