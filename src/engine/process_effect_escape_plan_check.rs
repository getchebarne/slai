use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;

// EscapePlan post-draw check: if the last card drawn (set by CardDraw) is a
// Skill, gain `block`. Consumes `card_last_drawn` so it can't fire on stale
// state if it runs again later
pub fn process_effect_escape_plan_check(state: &mut GameState, block: u16) {
    let id_card = match state.card_last_drawn.take() {
        Some(id) => id,
        None => return,
    };
    if state.entities[id_card].card_kind != CardKind::Skill {
        return;
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::BlockGain { amount: block },
        id_source: Some(state.id_character),
        target: Target::Direct(Some(state.id_character)),
    });
}
