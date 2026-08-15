use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::Frame;
use crate::utils::frame_top_mut;

// Discard every non-Attack Card from hand
pub fn process_effect_unload_discard(state: &mut GameState) {
    let Frame::Combat { id_hand, .. } = frame_top_mut(&mut state.frame_stack) else {
        unreachable!("process_effect_unload_discard outside the Combat frame")
    };
    for i in 0..id_hand.len() {
        let id_card = id_hand[i];
        if state.entities[id_card].card_kind != CardKind::Attack {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDiscard {
                    source: DiscardSource::Explicit, // Triggers on-discard sinergies
                },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }
}
