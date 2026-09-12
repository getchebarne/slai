use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At combat start, discard any number of Cards, then draw that many
// See:
//    - `process_effect_gamble.rs`
pub static GAMBLING_CHIP: RelicTemplate = RelicTemplate {
    name: RelicName::GamblingChip,
    tier: RelicTier::Rare,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[Effect {
        kind: EffectKind::Gamble {
            choose_discards: true,
            discards_before: None,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
