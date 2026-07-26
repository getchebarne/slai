use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Mode;

pub fn process_effect_card_draw_up_to(state: &mut GameState, amount: u8) {
    let Mode::Combat { id_hand, .. } = &mut state.mode else {
        unreachable!("process_effect_card_draw_up_to outside Combat mode")
    };
    let num_cards_to_draw = (amount as u16).saturating_sub(id_hand.len() as u16);
    if num_cards_to_draw > 0 {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDraw {
                count: num_cards_to_draw,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
