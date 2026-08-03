use crate::effect::Effect;
use crate::entity::Entity;
use crate::events::EFFECT_DECK_PURGE_PICK;
use crate::events::EFFECT_DECK_TRANSFORM_PICK;
use crate::events::EFFECT_DECK_UPGRADE_PICK;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_non_basic_non_curse;
use crate::events::deck_has_purgeable;
use crate::events::deck_has_upgradable;
use crate::events::make_entity_event_option;
use crate::game::GameState;

// Forget
const OPTION_FORGET: &[Effect] = &[EFFECT_DECK_PURGE_PICK, EVENT_CONSUME_EFFECT];

// Change
const OPTION_CHANGE: &[Effect] = &[EFFECT_DECK_TRANSFORM_PICK, EVENT_CONSUME_EFFECT];

// Grow
const OPTION_GROW: &[Effect] = &[EFFECT_DECK_UPGRADE_PICK, EVENT_CONSUME_EFFECT];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Forget] Remove a card from your deck.", OPTION_FORGET),
    make_entity_event_option("[Change] Transform a card in your deck.", OPTION_CHANGE),
    make_entity_event_option("[Grow] Upgrade a card in your deck.", OPTION_GROW),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => deck_has_non_basic_non_curse(state),
        2 => deck_has_upgradable(state),
        _ => unreachable!("Living wall option out of range: {idx}"),
    }
}
