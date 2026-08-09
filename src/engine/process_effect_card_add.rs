use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::utils::place_card;
use crate::utils::push_entity;

pub fn process_effect_card_add(
    state: &mut GameState,
    card_name: CardName,
    pile: CardPile,
    count: u16,
    upgraded: bool,
) {
    // Deck additions route through the obtain hook, one effect per Card
    if pile == CardPile::Deck {
        for _ in 0..count {
            let card = get_card(card_name, upgraded);
            let id_card = push_entity(&mut state.entities, card);
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardAddToDeck,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
        return;
    }

    for _ in 0..count {
        let card = get_card(card_name, upgraded);
        let id_card = push_entity(&mut state.entities, card);
        place_card(state, id_card, pile);
    }
}
