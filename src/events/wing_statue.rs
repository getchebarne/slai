use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::deck_has_damage_card;
use crate::events::deck_has_purgeable;
use crate::events::opt;
use crate::game::GameState;
use crate::types::DeltaSign;

// Pray
const OPTION_PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(7),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EFFECT_DECK_PURGE_PICK_1,
    EVENT_CONSUME_EFFECT,
];

// Attack
const OPTION_ATTACK: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Range { min: 50, max: 80 },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
pub static OPTIONS: &[&[Effect]] = &[opt(OPTION_PRAY), opt(OPTION_ATTACK), OPTION_LEAVE];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => deck_has_damage_card(state, 10),
        2 => true,
        _ => unreachable!("Wing statue option out of range: {idx}"),
    }
}
