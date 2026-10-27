use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;
use crate::utils::card_damage_delta;

// Mutates Glass Knife's own DamagePhysical amounts by `delta` in-place
pub fn process_effect_glass_knife_decay(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    delta: i16,
) {
    let id_target = first_target(
        state,
        EffectKind::GlassKnifeDecay { delta },
        id_source,
        id_targets,
    );
    let id_target = id_target.expect("GlassKnifeDecay requires id_target");
    card_damage_delta(&mut state.entities[id_target], delta);
}
