use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::relics::make_entity_relic;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static LANTERN: Entity = make_entity_relic(
    RelicName::Lantern,
    RelicTier::Common,
    0,
    &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 1,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
);
