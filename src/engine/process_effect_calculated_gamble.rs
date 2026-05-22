use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

pub fn process_effect_calculated_gamble(state: &mut GameState) {
    let num_cards = state.id_hand.len();
    // Draw runs after discards; push_front reverses, so push it first
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardDraw {
            count: num_cards as u16,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    // Discards in original order: iterate reverse, push_front
    for i in (0..state.id_hand.len()).rev() {
        let id_card = state.id_hand[i];
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
}
