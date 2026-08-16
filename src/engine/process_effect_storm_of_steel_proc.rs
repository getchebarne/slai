use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
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
    let Combat { id_card_hand, .. } = &mut state.combat;

    // Executes in reverse:
    //     1. CardDiscard (whole hand)
    //     2. CardAdd (Shivs)
    let count = id_card_hand.len() as u16;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Shiv,
            pile: CardPile::Hand,
            count,
            upgraded,
        },
        id_source: None,
        target: Target::Direct(None),
    });

    // Discard the current Cards
    for idx in 0..id_card_hand.len() {
        let id_card = id_card_hand[idx];
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit, // Triggers on-discard sinergies
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
}
