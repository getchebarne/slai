use crate::game::GameState;

pub fn process_effect_block_set(id_target: Option<usize>, state: &mut GameState, amount: u16) {
    let id_target = id_target.expect("BlockSet requires id_target");
    state.entities[id_target].vitals.block = amount;
}
