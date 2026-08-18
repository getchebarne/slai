use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::potions::PotionTemplate;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_GHOST_IN_A_JAR: PotionTemplate = PotionTemplate {
    name: PotionName::GhostInAJar,
    rarity: PotionRarity::Rare,
    combat_only: true,
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Intangible,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
