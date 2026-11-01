use crate::game::GameState;

// Links the Sapphire Key to the last staged Relic: queued right behind the chest's own roll, that's the chest's own
pub fn process_effect_key_sapphire_link(state: &mut GameState) {
    let id_relic = *state
        .reward
        .id_relics
        .last()
        .expect("The chest's own Relic is staged");
    state.reward.id_relic_linked = Some(id_relic);
}
