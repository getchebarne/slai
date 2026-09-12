use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th Attack played grants 1 energy; counter persists across turns and combats
// See:
//    - `process_effect_card_play.rs`
pub static NUNCHAKU: RelicTemplate = RelicTemplate {
    name: RelicName::Nunchaku,
    tier: RelicTier::Common,
    counter_init: 0,
    counter_reset: 10,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 1,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
