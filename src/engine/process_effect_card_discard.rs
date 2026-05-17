use std::collections::VecDeque;

use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::types::Phase;
use crate::utils::remove_card_from_collection;

// Unified discard handler. `source` selects the branch:
//
// - Explicit (Acrobatics, Concentrate, CalculatedGamble, Unload,
//   ToolsOfTheTrade, Storm of Steel, player-picked AwaitHandSelect, ...):
//   move hand -> discard, bump `this_turn_discards`, fire on-discard effects
//
// - End of turn: turn-end auto-discard. Honors `card_retain` (clear flag, stay
//   in hand) and `card_ethereal` (route to exhaust). Otherwise just moves
//   hand -> discard with no counter and no on-discard effects
//
// For the "move just-played card to discard pile" case (post-play), see
// process_effect_card_move_to_discard — that's not a discard
pub fn process_effect_card_discard(
    source: DiscardSource,
    id_target: usize,
    entities: &mut [Entity],
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    this_turn_discards: &mut u8,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    match source {
        DiscardSource::EndOfTurn => {
            if entities[id_target].card_retain {
                entities[id_target].card_retain = false;
                return None;
            }
            if entities[id_target].card_ethereal {
                effect_queue.push_front(Effect {
                    kind: EffectKind::CardExhaust,
                    id_source: None,
                    target: Target::Direct(Some(id_target)),
                });
                return None;
            }
            remove_card_from_collection(id_target, id_hand);
            id_pile_discard.push(id_target);
            None
        }
        DiscardSource::Explicit => {
            remove_card_from_collection(id_target, id_hand);
            id_pile_discard.push(id_target);
            *this_turn_discards = this_turn_discards.saturating_add(1);

            // Push in reverse so the first effect in the array runs first
            // when the effect_queue resumes
            let effects_on_discard = entities[id_target].card_on_discard_effects;
            for effect in effects_on_discard.iter().rev() {
                effect_queue.push_front(Effect {
                    id_source: Some(id_target),
                    ..*effect
                });
            }
            None
        }
    }
}
