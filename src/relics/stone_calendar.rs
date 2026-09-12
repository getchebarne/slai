use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// At the end of turn 7 each combat, deal 52 damage to all enemies
// See:
//    - `process_effect_turn_end.rs`
pub static STONE_CALENDAR: RelicTemplate = RelicTemplate {
    name: RelicName::StoneCalendar,
    tier: RelicTier::Rare,
    counter_init: 0,
    counter_reset: 7,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[Effect {
        kind: EffectKind::DamageDeal {
            amount: 52,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
};
