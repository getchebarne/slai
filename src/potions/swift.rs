use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::potions::make_entity_potion;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_SWIFT: Entity = make_entity_potion(
    PotionName::SwiftPotion,
    PotionRarity::Common,
    true,
    &[Effect {
        kind: EffectKind::CardDraw { count: 3 },
        id_source: None,
        target: Target::Direct(None),
    }],
);
