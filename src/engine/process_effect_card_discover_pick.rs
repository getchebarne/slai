use crate::consts::MAX_SIZE_HAND;
use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_card_discover_pick(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_card_discover_pick outside Combat mode")
    };
    let id_card = id_target.expect("CardDiscoverPick Direct form must have target");
    state.entities[id_card].card_free_to_play_once = true;
    if combat.id_hand.len() < MAX_SIZE_HAND {
        combat.id_hand.push(id_card);
    } else {
        combat.id_pile_discard.push(id_card);
    }
    combat.id_discover.clear();
}
