use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::health_delta;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Reach in; +1 HP and +10% per miss until the 105% rung, which cannot fail
const fn reach(dmg: u16, chance: u8, advance_on_miss: bool) -> [Effect; 2] {
    [
        health_delta(dmg),
        Effect {
            kind: EffectKind::ScrapOozeReach {
                chance,
                advance_on_miss,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}
const OPTIONS_REACH_BASE: [[Effect; 2]; 9] = [
    reach(3, 25, true),
    reach(4, 35, true),
    reach(5, 45, true),
    reach(6, 55, true),
    reach(7, 65, true),
    reach(8, 75, true),
    reach(9, 85, true),
    reach(10, 95, true),
    reach(11, 105, false),
];

// Base damage 3 -> 5 at A15
const OPTIONS_REACH_A15: [[Effect; 2]; 9] = [
    reach(5, 25, true),
    reach(6, 35, true),
    reach(7, 45, true),
    reach(8, 55, true),
    reach(9, 65, true),
    reach(10, 75, true),
    reach(11, 85, true),
    reach(12, 95, true),
    reach(13, 105, false),
];

// Leave
static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTIONS_REACH_BASE[0]),
    make_event_option_template(&OPTIONS_REACH_BASE[1]),
    make_event_option_template(&OPTIONS_REACH_BASE[2]),
    make_event_option_template(&OPTIONS_REACH_BASE[3]),
    make_event_option_template(&OPTIONS_REACH_BASE[4]),
    make_event_option_template(&OPTIONS_REACH_BASE[5]),
    make_event_option_template(&OPTIONS_REACH_BASE[6]),
    make_event_option_template(&OPTIONS_REACH_BASE[7]),
    make_event_option_template(&OPTIONS_REACH_BASE[8]),
    EOT_LEAVE,
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTIONS_REACH_A15[0]),
    make_event_option_template(&OPTIONS_REACH_A15[1]),
    make_event_option_template(&OPTIONS_REACH_A15[2]),
    make_event_option_template(&OPTIONS_REACH_A15[3]),
    make_event_option_template(&OPTIONS_REACH_A15[4]),
    make_event_option_template(&OPTIONS_REACH_A15[5]),
    make_event_option_template(&OPTIONS_REACH_A15[6]),
    make_event_option_template(&OPTIONS_REACH_A15[7]),
    // Base game also reads 105%. I'm innocent
    make_event_option_template(&OPTIONS_REACH_A15[8]),
    EOT_LEAVE,
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    let attempts = state.event.stage;
    match idx {
        0..=8 => idx as u8 == attempts,
        9 => true,
        _ => unreachable!("Scrap ooze option out of range: {idx}"),
    }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
