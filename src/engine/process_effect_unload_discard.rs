use crate::effect::DiscardSource;
use crate::effect::effect_direct;
use crate::effect::EffectKind;
use crate::game::GameState;
use crate::types::CardKind;

// Discard every non-Attack card from hand
pub fn process_effect_unload_discard(state: &mut GameState) {
    for i in 0..state.id_hand.len() {
        let id_card = state.id_hand[i];
        if state.entities[id_card].card_kind != CardKind::Attack {
            state.effect_queue.push_front(effect_direct(
                EffectKind::CardDiscard {
                    source: DiscardSource::Explicit,
                },
                None,
                Some(id_card),
            ));
        }
    }
}
