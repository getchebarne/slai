use crate::effect::Effect;
use crate::entity::Entity;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::deck_has_purgeable;
use crate::events::make_entity_event_option;
use crate::game::GameState;

// Pray
const OPTION_PRAY: &[Effect] = &[EFFECT_DECK_PURGE_PICK_1, EVENT_CONSUME_EFFECT];

// Leave
pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Pray] Remove a card from your deck.", OPTION_PRAY),
    OPTION_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => true,
        _ => unreachable!("Purifier option out of range: {idx}"),
    }
}
