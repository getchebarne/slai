use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every 3rd Skill played each turn deals 5 damage to all enemies
// See:
//    - `process_effect_card_play.rs`
pub static LETTER_OPENER: RelicTemplate = RelicTemplate {
    name: RelicName::LetterOpener,
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
        kind: EffectKind::DamageDeal {
            amount: 5,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
};
