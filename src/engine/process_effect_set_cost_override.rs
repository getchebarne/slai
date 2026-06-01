use crate::game::GameState;

pub fn process_effect_set_cost_override(
    id_target: Option<usize>,
    state: &mut GameState,
    amount: u8,
) {
    let id_target = id_target.expect("SetCostOverride requires id_target");
    state.entities[id_target].card_cost_override = Some(amount);
}
