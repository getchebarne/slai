use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Branches on `source`: Explicit bumps counter and fires on-discard; EndOfTurn honors retain/ethereal
pub fn process_effect_card_discard(
    id_target: Option<usize>,
    state: &mut GameState,
    source: DiscardSource,
) {
    let id_target = id_target.expect("CardDiscard requires id_target");
    match source {
        DiscardSource::EndOfTurn => {
            if state.entities[id_target].card_retain {
                state.entities[id_target].card_retain = false;
                return;
            }
            if state.entities[id_target].card_ethereal {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardExhaust,
                    id_source: None,
                    target: Target::Direct(Some(id_target)),
                });
                return;
            }
            if let Some(pos) = state.id_hand.iter().position(|&v| v == id_target) {
                state.id_hand.remove(pos);
            }
            state.id_pile_discard.push(id_target);
        }
        DiscardSource::Explicit => {
            if let Some(pos) = state.id_hand.iter().position(|&v| v == id_target) {
                state.id_hand.remove(pos);
            }
            state.id_pile_discard.push(id_target);
            state.this_turn_discards = state.this_turn_discards.saturating_add(1);

            // Push reversed so first effect runs first when queue resumes
            let effects_on_discard = state.entities[id_target].card_on_discard_effects;
            for effect in effects_on_discard.iter().rev() {
                state.effect_queue.push_front(Effect {
                    id_source: Some(id_target),
                    ..*effect
                });
            }
        }
    }
}
