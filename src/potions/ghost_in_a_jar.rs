use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_GHOST_IN_A_JAR: Entity = make_entity_potion(
    PotionName::GhostInAJar,
    PotionRarity::Rare,
    true,
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Intangible,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
);
