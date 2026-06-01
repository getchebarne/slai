use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;

// Discard every non-Attack card from hand
pub fn process_effect_unload_discard(state: &mut GameState) {
    for i in 0..state.id_hand.len() {
        let id_card = state.id_hand[i];
        if state.entities[id_card].card_kind != CardKind::Attack {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDiscard {
                    source: DiscardSource::Explicit,
                },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }
}
