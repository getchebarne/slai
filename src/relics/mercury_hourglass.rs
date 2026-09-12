use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// Every turn start deals 3 damage to all enemies
// See:
//    - `process_effect_turn_start.rs`
pub static MERCURY_HOURGLASS: RelicTemplate = RelicTemplate {
    name: RelicName::MercuryHourglass,
    tier: RelicTier::Uncommon,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    // Every turn: 3 damage to all Monsters
    effects_turn_start: &[Effect {
        kind: EffectKind::DamageDeal {
            amount: 3,
            lifesteal: false,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[],
    effects_on_rest: &[],
    effects_counter: &[],
};
