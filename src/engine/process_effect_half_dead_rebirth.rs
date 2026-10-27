use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;

// A half-dead Monster's rebirth starts here: from now on hits, Powers, Cards and random rolls reach it again
pub fn process_effect_half_dead_rebirth(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::HalfDeadRebirth, id_source, id_targets);
    let id_monster = id_target.expect("HalfDeadRebirth requires id_target");
    state.entities[id_monster].monster_half_dead = false;
}
