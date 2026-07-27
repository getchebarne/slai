use crate::game::GameState;
use crate::types::CardPile;
use crate::utils::detach_card;
use crate::utils::place_card;

pub fn process_effect_card_move(id_target: Option<usize>, state: &mut GameState, pile: CardPile) {
    // Relocation only — no discard / draw triggers fire
    let id_target = id_target.expect("CardMove requires id_target");
    detach_card(&mut state.mode, id_target);
    place_card(state, id_target, pile);
}
