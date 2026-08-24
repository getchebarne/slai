use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_damage_card;
use crate::events::deck_has_purgeable;
use crate::events::make_event_option_template;
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
    EFFECT_EVENT_CONSUME,
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
    EFFECT_EVENT_CONSUME,
];

// Leave
pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_PRAY),
    make_event_option_template(OPTION_ATTACK),
    EOT_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => deck_has_damage_card(state, 10),
        2 => true,
        _ => unreachable!("Wing statue option out of range: {idx}"),
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
