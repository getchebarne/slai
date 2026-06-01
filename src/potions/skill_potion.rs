use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::potions::EFFECT_CARD_DISCOVER_PICK;
use crate::types::CardKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static SKILL_POTION: Entity = make_entity_potion(
    PotionName::SkillPotion,
    PotionRarity::Common,
    false,
    true,
    &[
        Effect {
            kind: EffectKind::CardDiscoverRoll {
                kind: CardKind::Skill,
                count: DISCOVER_PICK_COUNT,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_CARD_DISCOVER_PICK,
    ],
);
