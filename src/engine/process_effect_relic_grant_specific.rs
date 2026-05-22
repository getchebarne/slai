use crate::engine::process_effect_relic_grant_random::grant_relic;
use crate::game::GameState;
use crate::types::RelicName;

pub fn process_effect_relic_grant_specific(
    state: &mut GameState,
    name: RelicName,
    fallback_circlet: bool,
) {
    let owns_target = state.id_relics[name as usize].is_some();
    let target = match (owns_target, fallback_circlet) {
        (false, _) => name,
        (true, true) => RelicName::Circlet,
        (true, false) => return,
    };
    grant_relic(target, &mut state.id_relics, &mut state.entities);
}
