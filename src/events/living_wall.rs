use crate::effect::Effect;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_DECK_TRANSFORM_PICK_1;
use crate::events::EFFECT_DECK_UPGRADE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_purgeable;
use crate::events::deck_has_upgradable;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Forget
const OPTION_FORGET: &[Effect] = &[EFFECT_DECK_PURGE_PICK_1, EFFECT_EVENT_CONSUME];

// Change
const OPTION_CHANGE: &[Effect] = &[EFFECT_DECK_TRANSFORM_PICK_1, EFFECT_EVENT_CONSUME];

// Grow
const OPTION_GROW: &[Effect] = &[EFFECT_DECK_UPGRADE_PICK_1, EFFECT_EVENT_CONSUME];

// The last row is Grow's stand-in when nothing is purgeable
pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_FORGET),
    make_event_option_template(OPTION_CHANGE),
    make_event_option_template(OPTION_GROW),
    EOT_LEAVE,
];

// Forget and Change stay open with nothing to purge; their empty picks resolve to nothing
pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 | 1 => true,
        2 => deck_has_upgradable(state),
        _ => unreachable!("Living wall option out of range: {idx}"),
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

// With nothing purgeable, Grow upgrades nothing and just ends the event
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    let eots = catalog(state.ascension);
    let grow = if deck_has_purgeable(state) {
        eots[2]
    } else {
        EOT_LEAVE
    };
    let options = [eots[0], eots[1], grow];
    bake_options(state, &options)
}
