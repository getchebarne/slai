use crate::game::GameState;
use crate::types::Focus;
use crate::utils::context_focus;

pub fn process_effect_rest_site_consume(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::RestSite,
        "RestSiteConsume outside the RestSite context"
    );
    state.rest_site.consumed = true;
}
