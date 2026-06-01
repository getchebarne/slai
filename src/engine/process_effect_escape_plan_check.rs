use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;

// If last-drawn is a Skill, gain `block`; consumes id_card_last_drawn
pub fn process_effect_escape_plan_check(state: &mut GameState, block: u16) {
    let id_card = match state.id_card_last_drawn.take() {
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
