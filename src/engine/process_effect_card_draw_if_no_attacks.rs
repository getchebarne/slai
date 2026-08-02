use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::Mode;
use crate::utils::mode_top;

pub fn process_effect_card_draw_if_no_attacks(state: &mut GameState, count: u16) {
    let Mode::Combat { id_hand, .. } = mode_top(&state.mode_stack) else {
        unreachable!("process_effect_card_draw_if_no_attacks outside Combat mode")
    };
    let any_attack = id_hand
        .iter()
        .any(|&id| state.entities[id].card_kind == CardKind::Attack);
    if !any_attack {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw { count },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
