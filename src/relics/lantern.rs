use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_relic;
use crate::types::RelicName;
use crate::types::RelicTier;

pub static LANTERN: Entity = make_entity_relic(
    RelicName::Lantern,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::EnergyGain { amount: 1 },
        id_source: None,
        target: Target::Direct(None),
    }],
);
