use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardPile;
use crate::utils::push_entity;

pub fn process_effect_card_duplicate(
    id_target: Option<usize>,
    state: &mut GameState,
    pile: CardPile,
) {
    let id_card = id_target.expect("CardDuplicate requires id_target");
    let mut card_copy = state.entities[id_card];

    // A combat copy keeps every stat of the original
    if pile != CardPile::Deck {
        let id_card_copy = push_entity(&mut state.entities, card_copy);
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardPlace { pile },
            id_source: None,
            target: Target::Direct(Some(id_card_copy)),
        });
        return;
    }

    // A deck copy leaves the bottle to the original
    card_copy.card_bottled = false;

    // Push it
    let id_card_copy = push_entity(&mut state.entities, card_copy);

    // Push `CardAdopt` so that on-card-add Effects trigger (e.g., Ceramic Fish)
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardAdopt { landing: false },
        id_source: None,
        target: Target::Direct(Some(id_card_copy)),
    });
}
