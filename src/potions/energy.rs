use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_potion;
use crate::types::DeltaSign;
use crate::types::PotionName;
use crate::types::PotionRarity;

pub static POTION_ENERGY: Entity = make_entity_potion(
    PotionName::EnergyPotion,
    PotionRarity::Common,
    false,
    true,
    &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 2,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
