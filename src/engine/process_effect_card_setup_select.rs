use crate::game::GameState;
use crate::types::Combat;

pub fn process_effect_card_setup_select(
    id_targets: &[usize],
    state: &mut GameState,
    free: bool,
    bottom: bool,
) {
    assert!(
        state.combat.active,
        "process_effect_card_setup_select outside the Combat frame"
    );
    assert!(!id_targets.is_empty(), "CardSetupSelect requires id_target");
    let Combat {
        id_card_pile_hand,
        id_card_pile_draw,
        ..
    } = &mut state.combat;
    for &id_target in id_targets {
        // Only a Card costing above 0 is made free; X-cost and unplayable Cards store cost 0
        if free && state.entities[id_target].card_cost > 0 {
            state.entities[id_target].card_free_to_play_once = true;
        }

        // A Card moved into the draw pile costs its combat cost again this turn; free-to-play-once survives
        state.entities[id_target].card_cost_override = None;

        if let Some(pos) = id_card_pile_hand.iter().position(|&id| id == id_target) {
            id_card_pile_hand.remove(pos);
        }

        // Top of the draw pile (Setup) is the vec's end; bottom (Forethought) is index 0
        if bottom {
            id_card_pile_draw.insert(0, id_target);
        } else {
            id_card_pile_draw.push(id_target);
        }
    }
}
