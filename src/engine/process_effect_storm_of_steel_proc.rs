use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::Mode;

// Discard the entire hand, then add 1 Shiv per discarded card
pub fn process_effect_storm_of_steel_proc(state: &mut GameState, upgraded: bool) {
    let Mode::Combat { id_hand, .. } = &mut state.mode else {
        unreachable!("process_effect_storm_of_steel_proc outside Combat mode")
    };

    // Executes in reverse:
    //     1. CardDiscard (whole hand)
    //     2. CardAdd (Shivs)
    let count = id_hand.len() as u16;
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

    // Discard the current cards
    for i in 0..id_hand.len() {
        let id_card = id_hand[i];
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit, // Triggers on-discard sinergies
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
}
