use crate::consts::MAX_SIZE_HAND;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Mode;

// Two-phase: `None` snapshots the discard counter and opens the any-number discard
// pick; `Some(before)` draws one card per discard made
pub fn process_effect_gambling_chip_proc(state: &mut GameState, discards_before: Option<u8>) {
    let Mode::Combat {
        this_turn_discards, ..
    } = &state.mode
    else {
        unreachable!("GamblingChipProc outside Combat mode")
    };

    match discards_before {
        None => {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::GamblingChipProc {
                    discards_before: Some(*this_turn_discards),
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
                    candidate_pool: CandidatePool::Hand {
                        filter: CandidatePoolCardFilter::Any,
                    },
                    selection_kind: SelectionKind::InputUpTo {
                        count: MAX_SIZE_HAND as u16,
                    },
                },
            });
        }
        Some(before) => {
            let count = this_turn_discards.saturating_sub(before) as u16;
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
