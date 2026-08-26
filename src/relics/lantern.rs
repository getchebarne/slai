use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// See:
//    - `process_effect_combat_start.rs`
pub static LANTERN: RelicTemplate = RelicTemplate {
    name: RelicName::Lantern,
    tier: RelicTier::Common,
    counter_init: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 1,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
