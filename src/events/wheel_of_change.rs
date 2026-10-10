use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Spin; consume first, so the result (a purge pick included) lands over the spent event
const OPTION_SPIN: &[Effect] = &[
    EFFECT_EVENT_CONSUME,
    Effect {
        kind: EffectKind::WheelSpin,
        id_source: None,
        target: Target::Direct(None),
    },
];

// Spin is mandatory, there's no "Leave" option
pub static EOTS_BASE: &[EventOptionTemplate] = &[make_event_option_template(OPTION_SPIN)];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
