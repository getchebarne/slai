use crate::game::GameState;

pub fn process_effect_rest_site_consume(id_target: Option<usize>, state: &mut GameState) {
    let id_room = id_target.expect("RestSiteConsume requires id_target");
    state.entities[id_room].room_rest_site_done = true;
}
