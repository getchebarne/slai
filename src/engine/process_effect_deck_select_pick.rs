use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::game::GameState;
use crate::types::DeckSelectKind;

// Direct form: apply the deck-pick action by kind. Push effects to queue front
pub fn process_effect_deck_select_pick(
    id_target: Option<usize>,
    state: &mut GameState,
    kind: DeckSelectKind,
) {
    let id_card = id_target.expect("DeckSelectPick Direct form must have target");
    match kind {
        DeckSelectKind::Remove => {
            state.effect_queue.push_front(Effect::direct(
                EffectKind::CardRemoveFromDeck,
                None,
                Some(id_card),
            ));
        }
        DeckSelectKind::UpgradeAny => {
            state.effect_queue.push_front(Effect::direct(
                EffectKind::CardUpgrade,
                None,
                Some(id_card),
            ));
        }
        DeckSelectKind::DuplicateAny => {
            let card = &state.entities[id_card];
            let card_name = card.card_name;
            let upgraded = card.card_upgraded;
            state.effect_queue.push_front(Effect::direct(
                EffectKind::CardAddToDeck {
                    card_name,
                    upgraded,
                },
                None,
                None,
            ));
        }
        DeckSelectKind::TransformOne => {
            state.effect_queue.push_front(Effect::direct(
                EffectKind::CardTransformRoll,
                None,
                None,
            ));
            state.effect_queue.push_front(Effect::direct(
                EffectKind::CardRemoveFromDeck,
                None,
                Some(id_card),
            ));
        }
    }
}
