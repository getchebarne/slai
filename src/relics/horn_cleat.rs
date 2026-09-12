use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the start of turn 2 each combat, gain 14 block
// See:
//    - `process_effect_turn_start.rs`
pub static HORN_CLEAT: RelicTemplate = RelicTemplate {
    name: RelicName::HornCleat,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 2,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    // Relic-sourced block: id_source None skips Dex / Frail scaling
    effects_counter: &[Effect {
        kind: EffectKind::BlockGain { amount: 14 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
