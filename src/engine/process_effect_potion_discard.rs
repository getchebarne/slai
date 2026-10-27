use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;
use crate::potions::remove_potion;

pub fn process_effect_potion_discard(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::PotionDiscard, id_source, id_targets);
    let id_potion = id_target.expect("PotionDiscard requires id_target");
    remove_potion(&mut state.id_potions, id_potion);
}
