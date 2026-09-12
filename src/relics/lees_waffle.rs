use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 7 max HP and heal to full
// See:
//    - `process_effect_relic_adopt.rs`
pub static LEES_WAFFLE: RelicTemplate = RelicTemplate {
    name: RelicName::LeesWaffle,
    tier: RelicTier::Shop,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    // +7 max HP, then heal to full
    effects_on_pickup: &[
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(7),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 1,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    effects_on_rest: &[],
    effects_counter: &[],
};
