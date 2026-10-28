use crate::game::GameState;
use crate::utils::card_damage_delta;

// Mutates Glass Knife's own DamagePhysical amounts by `delta` in-place
pub fn process_effect_glass_knife_decay(
    id_target: Option<usize>,
    state: &mut GameState,
    delta: i16,
) {
    let id_target = id_target.expect("GlassKnifeDecay requires id_target");
    card_damage_delta(&mut state.entities[id_target], delta);
}
