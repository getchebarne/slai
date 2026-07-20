use crate::game::GameState;
use crate::types::Mode;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_pick(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_card_nightmare_pick outside Combat mode")
    };
    let id_target = id_target.expect("CardNightmarePick requires id_target");
    let card = state.entities[id_target];
    let id = push_entity(&mut state.entities, card);
    combat.id_card_nightmare = Some(id);
}
