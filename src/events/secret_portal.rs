use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CLOSE;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::game::Location;

// Enter takes the portal at once: the event closes first, so the boss fight is the Boss Room's, not the event's
const OPTION_ENTER: [Effect; 2] = [
    EFFECT_EVENT_CLOSE,
    Effect {
        kind: EffectKind::RoomEnter {
            location: Location::BossRoom,
            landing: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
];

static EOTS_BASE: &[EventOptionTemplate] = &[make_event_option_template(&OPTION_ENTER), EOT_LEAVE];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
