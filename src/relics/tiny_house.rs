use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RewardRollTrigger;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup: upgrade 1 random Card, +5 max HP (healed), 50 gold, 1 random Potion, 1 Card
// See:
//    - `process_effect_relic_adopt.rs`
pub static TINY_HOUSE: RelicTemplate = RelicTemplate {
    name: RelicName::TinyHouse,
    tier: RelicTier::Boss,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_pickup: &[Effect {
        kind: EffectKind::RewardRollCards {
            bundles: 1,
            trigger: RewardRollTrigger::CombatMonster,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_rest: &[],
    effects_counter: &[],
};
