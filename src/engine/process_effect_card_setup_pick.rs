use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_card_setup_pick(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_card_setup_pick outside Combat mode")
    };
    let id_target = id_target.expect("CardSetupPick requires id_target");
    state.entities[id_target].card_free_to_play_once = true;
    if let Some(pos) = combat.id_hand.iter().position(|&v| v == id_target) {
        combat.id_hand.remove(pos);
    }
    combat.id_pile_draw.push(id_target);
}
