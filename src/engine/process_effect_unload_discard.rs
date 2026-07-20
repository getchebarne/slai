use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::Mode;

// Discard every non-Attack card from hand
pub fn process_effect_unload_discard(state: &mut GameState) {
    let Mode::Combat(combat) = &mut state.mode else {
        unreachable!("process_effect_unload_discard outside Combat mode")
    };
    for i in 0..combat.id_hand.len() {
        let id_card = combat.id_hand[i];
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
