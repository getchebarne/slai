use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_GAMBLERS_BREW: Entity = make_entity_potion(
    PotionName::GamblersBrew,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::Gamble {
            choose_discards: true,
            discards_before: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
