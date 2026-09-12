use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 10th Card played draws 1 Card; counter persists across turns and combats
// See:
//    - `process_effect_card_play.rs`
pub static INK_BOTTLE: RelicTemplate = RelicTemplate {
    name: RelicName::InkBottle,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 10,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[Effect {
        kind: EffectKind::CardDraw { count: 1 },
        id_source: None,
        target: Target::Direct(None),
    }],
};
