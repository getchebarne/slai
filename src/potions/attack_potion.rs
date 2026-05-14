use crate::consts::DISCOVER_PICK_COUNT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::CardKind;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static ATTACK_POTION: Entity = make_entity_potion(
    PotionName::AttackPotion,
    PotionRarity::Common,
    false,
    true,
    &[Effect {
        kind: EffectKind::CardDiscoverPick {
            kind: CardKind::Attack,
            count: DISCOVER_PICK_COUNT,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
