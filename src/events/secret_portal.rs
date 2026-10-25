use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_ADVANCE;
use crate::events::EFFECT_EVENT_CLOSE;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::game::Location;

// Enter: a confirm screen whose one option takes the portal
const OPTION_ENTER: [Effect; 1] = [EFFECT_EVENT_ADVANCE];

// Take the portal: the event closes first, so the boss fight is the Boss Room's, not the event's
const OPTION_CONFIRM: [Effect; 2] = [
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

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_ENTER),
    EOT_LEAVE,
    make_event_option_template(&OPTION_CONFIRM),
];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

// Stages: 0 the intro (Enter, Leave), 1 the confirm
pub fn option_available(state: &GameState, idx: usize) -> bool {
    match state.event.stage {
        0 => idx == 0 || idx == 1,
        _ => idx == 2,
    }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
