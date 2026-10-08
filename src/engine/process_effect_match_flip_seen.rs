use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::game::GameState;
use crate::types::CardPile;
use crate::types::EventName;
use crate::types::Focus;
use crate::utils::context_focus;

// Match and Keep!: flips a face-down Card already seen, picked by the player or just revealed by an unseen flip
pub fn process_effect_match_flip_seen(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("MatchFlipSeen requires id_target");
    assert!(
        context_focus(state) == Focus::Event,
        "MatchFlipSeen outside the Event context"
    );
    assert!(
        state.event.name == EventName::MatchAndKeep,
        "MatchFlipSeen outside a Match and Keep! event"
    );
    let event = &mut state.event;

    // The flipped Card leaves the face-down Cards
    let idx = event
        .id_roll_card
        .iter()
        .position(|&id| id == id_card)
        .expect("A seen flip picks a seen Card");
    event.id_roll_card.remove(idx);

    // The attempt's first flip stays face up
    let Some(id_card_first) = event.id_match_flipped.take() else {
        event.id_match_flipped = Some(id_card);
        return;
    };

    // The second flip ends the attempt; a pair adds a fresh copy to the deck
    let card_name = state.entities[id_card].card_name;
    if state.entities[id_card_first].card_name == card_name {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CardAdd {
                card_name,
                pile: CardPile::Deck,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        });
    } else {
        // A miss goes back face down, among the seen Cards
        event.id_roll_card.push(id_card_first);
        event.id_roll_card.push(id_card);
    }
    event.match_attempts -= 1;

    // The event ends after the last attempt
    if event.match_attempts == 0 {
        state.effect_queue.push_back(EFFECT_EVENT_CONSUME);
    }
}
