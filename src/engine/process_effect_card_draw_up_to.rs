use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;

pub fn process_effect_card_draw_up_to(state: &mut GameState, amount: u8) {
    assert!(
        state.combat.active,
        "process_effect_card_draw_up_to outside the Combat frame"
    );
    let Combat { id_card_hand, .. } = &mut state.combat;
    let num_cards_to_draw = (amount as u16).saturating_sub(id_card_hand.len() as u16);
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
