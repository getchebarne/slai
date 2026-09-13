use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd reshuffle grants 2 energy; counter persists across combats
// See:
//    - `process_effect_shuffle_discard_pile_into_draw_pile.rs`
pub static SUNDIAL: RelicTemplate = RelicTemplate {
    name: RelicName::Sundial,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 3,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[],
    effects_rest: &[],
    effects_counter: &[Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 2,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
};
