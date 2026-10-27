use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::first_target;
use crate::game::GameState;
use crate::utils::flush_effects_from_buf_to_queue_front;

// Toke lands on the selected Card: it is purged, then the site is spent
pub fn process_effect_rest_toke(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::RestToke, id_source, id_targets);
    let id_card = id_target.expect("RestToke requires id_target");
    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::RestSiteConsume,
        id_source: None,
        target: Target::Direct(None),
    });
    flush_effects_from_buf_to_queue_front(state);
}
