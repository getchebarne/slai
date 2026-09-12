use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::relics::RelicTemplate;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::RelicTier;

// On pickup, gain 300 gold
// See:
//    - `process_effect_relic_adopt.rs`
pub static OLD_COIN: RelicTemplate = RelicTemplate {
    name: RelicName::OldCoin,
    tier: RelicTier::Rare,
    counter_init: 0,
    counter_reset: 0,
    effects_combat_start: &[],
    effects_turn_start: &[],
    effects_turn_end: &[],
    effects_combat_end: &[],
    effects_on_pickup: &[Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(300),
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    effects_on_rest: &[],
    effects_counter: &[],
};
