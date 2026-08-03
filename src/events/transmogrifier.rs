use crate::effect::Effect;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EFFECT_DECK_TRANSFORM_PICK;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::deck_has_non_basic_non_curse;
use crate::game::GameState;

// Pray
const OPTION_PRAY: &[Effect] = &[EFFECT_DECK_TRANSFORM_PICK, EVENT_CONSUME_EFFECT];

// Leave
pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Pray] Transform a card.", OPTION_PRAY),
    OPTION_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_non_basic_non_curse(state),
        1 => true,
        _ => unreachable!("Transmogrifier option out of range: {idx}"),
    }
}
