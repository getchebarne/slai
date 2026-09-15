use crate::cards::get_card;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_stacks;
use crate::types::CardName;
use crate::types::CardPile;
use crate::utils::card_damage_delta;
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
                kind: EffectKind::CardAdopt,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
        return;
    }

    // Accuracy: Shivs gain +stacks damage
    let accuracy_stacks = if card_name == CardName::Shiv && state.combat.active {
        modifier_stacks(
            &state.entities[state.id_character].modifiers,
            ModifierKind::Accuracy,
        )
    } else {
        0
    };

    for _ in 0..count {
        let card = get_card(card_name, upgraded);
        let id_card = push_entity(&mut state.entities, card);
        if accuracy_stacks != 0 {
            card_damage_delta(&mut state.entities[id_card], accuracy_stacks);
        }
        place_card(state, id_card, pile);
    }
}
