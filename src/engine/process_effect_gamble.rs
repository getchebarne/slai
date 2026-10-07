use crate::consts::MAX_SIZE_HAND;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;

// Two-phase discard-then-draw-that-many. Phase 1 (None) snapshots the discard
// counter and queues the discards: player-chosen (Gambler's Brew) or the whole
// hand (Calculated Gamble). Phase 2 draws the delta
pub fn process_effect_gamble(
    state: &mut GameState,
    choose_discards: bool,
    discards_before: Option<u16>,
) {
    assert!(
        state.combat.active,
        "process_effect_gamble outside the Combat frame"
    );
    let Combat {
        id_card_hand,
        this_turn_discards,
        ..
    } = &state.combat;
    match discards_before {
        None => {
            if id_card_hand.is_empty() {
                return;
            }
            let before = *this_turn_discards;
            // Draw phase runs after the discards; push_front reverses, so push it first
            state.effect_queue.push_front(Effect {
                kind: EffectKind::Gamble {
                    choose_discards,
                    discards_before: Some(before),
                },
                id_source: None,
                target: Target::Direct(None),
            });
            if choose_discards {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardDiscard {
                        source: DiscardSource::Explicit, // Triggers on-discard sinergies
                    },
                    id_source: None,
                    target: Target::Resolve {
                        candidate_pool: CandidatePool::Hand,
                        filter: CandidateFilter::Any,
                        selection_kind: SelectionKind::InputUpTo {
                            count: MAX_SIZE_HAND as u16,
                        },
                    },
                });
                return;
            }

            // The whole hand leaves from the right, so the leftmost Card ends on top of the discard pile
            for &id_card in id_card_hand.iter() {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardDiscard {
                        source: DiscardSource::Explicit, // Triggers on-discard sinergies
                    },
                    id_source: None,
                    target: Target::Direct(Some(id_card)),
                });
            }
        }
        Some(before) => {
            let count = this_turn_discards.saturating_sub(before);
            if count > 0 {
                state.effect_queue.push_front(Effect {
                    kind: EffectKind::CardDraw { count },
                    id_source: None,
                    target: Target::Direct(None),
                });
            }
        }
    }
}
