use crate::effect::Effect;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_DECK_TRANSFORM_PICK_1;
use crate::events::EFFECT_DECK_UPGRADE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_non_basic_non_curse;
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

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_FORGET),
    make_event_option_template(OPTION_CHANGE),
    make_event_option_template(OPTION_GROW),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => deck_has_non_basic_non_curse(state),
        2 => deck_has_upgradable(state),
        _ => unreachable!("Living wall option out of range: {idx}"),
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
