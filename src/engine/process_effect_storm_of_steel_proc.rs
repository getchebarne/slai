use crate::effect::DiscardSource;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;

// Discard the entire hand, then add 1 Shiv per discarded card
pub fn process_effect_storm_of_steel_proc(state: &mut GameState, upgraded: bool) {
    let n = state.id_hand.len() as u16;
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAddToHand {
            card_name: CardName::Shiv,
            count: n,
            upgraded,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    for i in 0..state.id_hand.len() {
        let id_card = state.id_hand[i];
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardDiscard {
                source: DiscardSource::Explicit, // Triggers on-discard sinergies
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
}
