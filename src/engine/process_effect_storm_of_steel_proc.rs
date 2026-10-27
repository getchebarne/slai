use crate::consts::MAX_SIZE_CARD_PILE_HAND;
use crate::effect::CandidatePool;
use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Combat;

// Discard the entire hand, then add 1 Shiv per discarded Card
pub fn process_effect_storm_of_steel_proc(state: &mut GameState, upgraded: bool) {
    assert!(
        state.combat.active,
        "process_effect_storm_of_steel_proc outside the Combat frame"
    );
    let Combat {
        id_card_pile_hand, ..
    } = &mut state.combat;

    // Executes in reverse:
    //     1. CardDiscard (whole hand)
    //     2. CardAdd (Shivs)
    let count = id_card_pile_hand.len() as u16;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shiv,
            card_pile: CardPile::Hand,
            count,
            upgraded,
        },
        id_source: None,
        target: Target::Direct(None),
    });

    // The whole hand leaves from the right, so the leftmost Card ends on top of the discard pile
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardDiscard {
            source: DiscardSource::Explicit, // Triggers on-discard synergies
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileHand,
            filters: &[],
            selection_kind: SelectionKind::Input {
                count: MAX_SIZE_CARD_PILE_HAND as u16,
            },
        },
    });
}
