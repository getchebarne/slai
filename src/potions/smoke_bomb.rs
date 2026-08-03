use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_SMOKE_BOMB: Entity = make_entity_potion(
    PotionName::SmokeBomb,
    PotionRarity::Rare,
    false,
    true,
    &[Effect {
        kind: EffectKind::CombatEnd {
            escaped_character: true,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
