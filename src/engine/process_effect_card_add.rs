use crate::cards::get_card;
use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
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
                kind: EffectKind::CardAdopt { landing: false },
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
        return;
    }

    let has_accuracy = has_modifier(
        &state.entities[state.id_character].modifiers,
        ModifierKind::Accuracy,
    );
    let mut landed_in_hand = false;
    for _ in 0..count {
        let card = get_card(card_name, upgraded);
        let id_card = push_entity(&mut state.entities, card);
        let placed = place_card(state, id_card, pile);
        landed_in_hand |= placed && pile == CardPile::Hand;

        // Accuracy: a new Shiv starts at its printed damage plus Accuracy, wherever it lands
        if has_accuracy && card_name == CardName::Shiv {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::AccuracyResync,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }

        // A created Eviscerate starts its cost this turn at its combat cost less this turn's discards
        if card.card_cost_kind == CardCostKind::MinusDiscardsThisTurn {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardCostMinusDiscards,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
    }

    // Accuracy: a Card reaching the hand resets every Shiv in it
    if landed_in_hand && has_accuracy {
        state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
    }
}
