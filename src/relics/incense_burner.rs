use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::modifier::ModifierKind;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 6th turn start grants 1 Intangible; counter persists across combats
// See:
//    - `process_effect_turn_start.rs`
pub static INCENSE_BURNER: RelicTemplate = RelicTemplate {
    name: RelicName::IncenseBurner,
    tier: RelicTier::Rare,
    counter_init: 0,
    counter_reset: 6,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Intangible,
            stacks: 1,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }],
};
