use crate::game::GameState;

pub fn process_effect_card_discover_pick(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardDiscoverPick Direct form must have target");
    state.entities[id_card].card_free_to_play_once = true;
    state.id_hand.push(id_card);
    state.id_discover.clear();
}
