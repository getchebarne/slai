use crate::game::GameState;
use crate::types::CardKind;
use crate::types::RelicName;

// Du-Vu Doll: the counter holds the deck's Curse count
pub fn process_effect_du_vu_doll_recount(state: &mut GameState) {
    let id =
        state.id_relics[RelicName::DuVuDoll as usize].expect("DuVuDollRecount requires Du-Vu Doll");
    let num_curses = state
        .id_card_pile_deck
        .iter()
        .filter(|&&id_card| state.entities[id_card].card_kind == CardKind::Curse)
        .count();
    state.entities[id].relic_counter = num_curses as i16;
}
