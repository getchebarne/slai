use crate::game::GameState;

pub fn process_effect_relic_adopt(id_target: Option<usize>, state: &mut GameState) {
    let id_relic = id_target.expect("RelicAdopt requires id_target");
    let name = state.entities[id_relic].relic_name;
    state.id_relics[name as usize] = Some(id_relic);
}
