use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;

// Bottled Cards join innate Cards at the top of the opening draw pile
pub fn process_effect_card_bottle(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::CardBottle, id_source, id_targets);
    let id_card = id_target.expect("CardBottle requires id_target");
    state.entities[id_card].card_bottled = true;
}
