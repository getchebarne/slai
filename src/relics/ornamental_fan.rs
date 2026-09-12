use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd Attack played each turn grants 4 block
// See:
//    - `process_effect_card_play.rs`
pub static ORNAMENTAL_FAN: RelicTemplate = RelicTemplate {
    name: RelicName::OrnamentalFan,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 3,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    // Relic-sourced block: id_source None skips Dex / Frail scaling
    effects_counter: &[Effect {
        kind: EffectKind::BlockGain { amount: 4 },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
