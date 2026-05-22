use crate::game::GameState;

pub fn process_effect_card_setup_pick(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardSetupPick requires id_target");
    state.entities[id_target].card_free_to_play_once = true;
    if let Some(pos) = state.id_hand.iter().position(|&v| v == id_target) {
        state.id_hand.remove(pos);
    }
    state.id_pile_draw.push(id_target);
}
