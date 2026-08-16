use crate::consts::POTION_SLOTS_MAX;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_ENTROPIC_BREW: Entity = make_entity_potion(
    PotionName::EntropicBrew,
    PotionRarity::Rare,
    false,
    &[Effect {
        kind: EffectKind::PotionAddRandom { limited: true },
        id_source: None,
        target: Target::Direct(None),
    }; POTION_SLOTS_MAX],
);
