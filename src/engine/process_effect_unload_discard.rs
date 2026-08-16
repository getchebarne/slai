use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardKind;
use crate::types::Combat;

// Discard every non-Attack Card from hand
pub fn process_effect_unload_discard(state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_unload_discard outside the Combat frame"
    );
    let Combat { id_card_hand, .. } = &mut state.combat;
    for idx in 0..id_card_hand.len() {
        let id_card = id_card_hand[idx];
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
