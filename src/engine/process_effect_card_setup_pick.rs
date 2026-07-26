use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::CostScope;
use crate::types::Mode;

pub fn process_effect_card_setup_pick(
    id_target: Option<usize>,
    state: &mut GameState,
    free: bool,
    bottom: bool,
) {
    let Mode::Combat {
        id_hand,
        id_pile_draw,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_card_setup_pick outside Combat mode")
    };
    let id_target = id_target.expect("CardSetupPick requires id_target");
    if free {
        state.entities[id_target].card_cost_override = Some(CostOverride {
            amount: 0,
            scope: CostScope::UntilPlayed,
        });
    }
    if let Some(pos) = id_hand.iter().position(|&v| v == id_target) {
        id_hand.remove(pos);
    }
    // Top of the draw pile is the vec's end; bottom (Forethought) is index 0
    if bottom {
        id_pile_draw.insert(0, id_target);
    } else {
        id_pile_draw.push(id_target);
    }
}
