use crate::cards::get_card;
use crate::game::GameState;

pub fn process_effect_card_upgrade(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("CardUpgrade requires id_target");

    // Already upgraded: skip so the swap doesn't wipe runtime state
    if state.entities[id_target].card_upgraded {
        return;
    }

    // Replace w/ upgraded variant
    let name = state.entities[id_target].card_name;
    state.entities[id_target] = get_card(name, true);
}
