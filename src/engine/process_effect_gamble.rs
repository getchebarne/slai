use crate::consts::MAX_SIZE_HAND;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Mode;
use crate::utils::mode_top;

// Two-phase discard-then-draw-that-many. Phase 1 (None) snapshots the discard
// counter and queues the discards: player-chosen (Gambler's Brew) or the whole
// hand (Calculated Gamble). Phase 2 draws the delta
pub fn process_effect_gamble(
    state: &mut GameState,
    choose_discards: bool,
    discards_before: Option<u8>,
) {
    let Mode::Combat {
        id_hand,
        this_turn_discards,
        ..
    } = mode_top(&state.mode_stack)
    else {
        unreachable!("process_effect_gamble outside Combat mode")
    };
    match discards_before {
        None => {
            if id_hand.is_empty() {
                return;
            }
            let before = *this_turn_discards;
            // Draw phase runs after the picks; push_front reverses, so push it first
            state.effect_queue.push_front(Effect {
                kind: EffectKind::Gamble {
                    choose_discards,
                    discards_before: Some(before),
                },
                id_source: None,
                target: Target::Direct(None),
            });
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardDiscard {
                    source: DiscardSource::Explicit, // Triggers on-discard sinergies
                },
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Hand,
                    filter: CandidateFilter::Any,
                    selection_kind: if choose_discards {
                        SelectionKind::InputUpTo {
                            count: MAX_SIZE_HAND as u16,
                        }
                    } else {
                        SelectionKind::All
                    },
                },
            });
        }
        Some(before) => {
            let count = this_turn_discards.saturating_sub(before);
            if count > 0 {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardDraw {
                        count: count as u16,
                    },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
    }
}
