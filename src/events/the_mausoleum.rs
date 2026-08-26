use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Open the casket: a Relic, and maybe Writhe
const OPTION_OPEN: &[Effect] = &[
    Effect {
        kind: EffectKind::MausoleumOpen,
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] =
    &[make_event_option_template(OPTION_OPEN), EOT_LEAVE];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
