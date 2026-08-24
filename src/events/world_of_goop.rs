use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;

// Gather
const OPTION_GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(11),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(75),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Leave
const fn leave(min: u16, max: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Range { min, max },
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Leave: forfeit a fifth of the gold
const OPTION_LEAVE_BASE: [Effect; 2] = leave(20, 50);

// Leave at A15+: forfeit 35%
const OPTION_LEAVE_A15: [Effect; 2] = leave(35, 75);

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_GATHER),
    make_event_option_template(&OPTION_LEAVE_BASE),
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_GATHER),
    make_event_option_template(&OPTION_LEAVE_A15),
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
