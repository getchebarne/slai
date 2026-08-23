use crate::effect::Effect;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_DECK_TRANSFORM_PICK_1;
use crate::events::EFFECT_DECK_UPGRADE_PICK_1;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_non_basic_non_curse;
use crate::events::deck_has_purgeable;
use crate::events::deck_has_upgradable;
use crate::events::opt;
use crate::game::GameState;

// Forget
const OPTION_FORGET: &[Effect] = &[EFFECT_DECK_PURGE_PICK_1, EVENT_CONSUME_EFFECT];

// Change
const OPTION_CHANGE: &[Effect] = &[EFFECT_DECK_TRANSFORM_PICK_1, EVENT_CONSUME_EFFECT];

// Grow
const OPTION_GROW: &[Effect] = &[EFFECT_DECK_UPGRADE_PICK_1, EVENT_CONSUME_EFFECT];

pub static OPTIONS: &[&[Effect]] = &[opt(OPTION_FORGET), opt(OPTION_CHANGE), opt(OPTION_GROW)];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        1 => deck_has_non_basic_non_curse(state),
        2 => deck_has_upgradable(state),
        _ => unreachable!("Living wall option out of range: {idx}"),
    }
}
