use crate::consts::MAX_SIZE_HAND;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;

// Discard, then draw that many: the discards are player-chosen (Gambler's Brew) or the whole hand (Calculated Gamble)
pub fn process_effect_gamble(state: &mut GameState, choose_discards: bool) {
    assert!(
        state.combat.active,
        "process_effect_gamble outside the Combat frame"
    );
    let Combat {
        id_card_hand,
        this_turn_discards,
        ..
    } = &state.combat;
    if id_card_hand.is_empty() {
        return;
    }

    // Draw phase runs after the discards; push_front reverses, so push it first
    state.effect_queue.push_front(Effect {
        kind: EffectKind::GambleDraw {
            discards_before: *this_turn_discards,
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
                filters: &[],
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
