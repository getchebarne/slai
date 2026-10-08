use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::flush_effects_from_buf_to_queue_front;

// Toke lands on the picked Card: it is purged, then the site is spent
pub fn process_effect_rest_toke(id_target: Option<usize>, state: &mut GameState) {
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
