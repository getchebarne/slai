use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;

// A Card's free play is spent without a play: a Confusion draw, an ethereal Card's end-of-turn exhaust, or a waiting draw-top Card the early turn end relocates
pub fn process_effect_card_free_play_spend(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::CardFreePlaySpend, id_source, id_targets);
    let id_card = id_target.expect("CardFreePlaySpend requires id_target");
    state.entities[id_card].card_free_to_play_once = false;
}
