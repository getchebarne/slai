use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_DISTILLED_CHAOS: Entity = make_entity_potion(
    PotionName::DistilledChaos,
    PotionRarity::Uncommon,
    false,
    true,
    &[Effect {
        kind: EffectKind::CardPlayFromDrawTop,
        id_source: None,
        target: Target::Direct(None),
    }; 3],
);
