use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static BLOCK: PotionTemplate = PotionTemplate {
    name: PotionName::Block,
    rarity: PotionRarity::Common,
    combat_only: true,
    doubled: false,
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 12 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
// Doubled
pub static BLOCK_DOUBLED: PotionTemplate = PotionTemplate {
    doubled: true,
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 24 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
    ..BLOCK
};
