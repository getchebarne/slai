use crate::effect::Effect;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_purgeable;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Pray
const OPTION_PRAY: &[Effect] = &[EFFECT_DECK_PURGE_PICK_1, EFFECT_EVENT_CONSUME];

// Leave
pub static EOTS_BASE: &[EventOptionTemplate] =
    &[make_event_option_template(OPTION_PRAY), EOT_LEAVE];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => true,
        _ => unreachable!("Purifier option out of range: {idx}"),
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
