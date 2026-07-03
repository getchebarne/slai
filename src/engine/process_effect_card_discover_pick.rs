use crate::consts::MAX_SIZE_HAND;
use crate::game::GameState;

pub fn process_effect_card_discover_pick(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("CardDiscoverPick Direct form must have target");
    state.entities[id_card].card_free_to_play_once = true;
    if state.id_hand.len() < MAX_SIZE_HAND {
        state.id_hand.push(id_card);
    } else {
        state.id_pile_discard.push(id_card);
    }
    state.id_discover.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::get_card;
    use crate::game::GameState;
    use crate::game::create_game_state;
    use crate::types::CardName;
    use crate::utils::push_entity;

    fn state_with_hand_size(n: usize) -> (GameState, usize) {
        let mut state = create_game_state(0, 42, false);
        assert!(state.id_hand.is_empty());
        for _ in 0..n {
            let id = push_entity(&mut state.entities, get_card(CardName::Backflip, false));
            state.id_hand.push(id);
        }
        let id_pick = push_entity(&mut state.entities, get_card(CardName::Neutralize, false));
        state.id_discover.push(id_pick);
        (state, id_pick)
    }

    #[test]
    fn pick_with_room_goes_to_hand() {
        let (mut state, id_pick) = state_with_hand_size(MAX_SIZE_HAND - 1);
        process_effect_card_discover_pick(Some(id_pick), &mut state);
        assert_eq!(state.id_hand.len(), MAX_SIZE_HAND);
        assert_eq!(state.id_hand.last(), Some(&id_pick));
        assert!(state.id_pile_discard.is_empty());
        assert!(state.id_discover.is_empty());
    }

    #[test]
    fn pick_with_full_hand_goes_to_discard() {
        let (mut state, id_pick) = state_with_hand_size(MAX_SIZE_HAND);
        process_effect_card_discover_pick(Some(id_pick), &mut state);
        assert_eq!(state.id_hand.len(), MAX_SIZE_HAND);
        assert_eq!(state.id_pile_discard.last(), Some(&id_pick));
        assert!(state.id_discover.is_empty());
    }
}
