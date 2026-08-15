use crate::game::GameState;
use crate::types::Frame;
use crate::utils::frame_top_mut;

pub fn process_effect_rest_site_consume(state: &mut GameState) {
    let Frame::RestSite { consumed } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("RestSiteConsume outside the RestSite frame")
    };
    *consumed = true;
}
