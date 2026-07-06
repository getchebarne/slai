use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::DeltaSign;

pub fn process_effect_card_purge(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardPurge requires id_target");

    // Parasite costs 3 max HP when removed from the master deck
    if state.entities[id_card].card_name == CardName::Parasite {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Fixed(3),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }
    if let Some(pos) = state.id_deck.iter().position(|&v| v == id_card) {
        state.id_deck.remove(pos);
    }
}
