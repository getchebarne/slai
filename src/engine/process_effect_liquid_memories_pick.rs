use crate::consts::MAX_SIZE_HAND;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::CostScope;
use crate::types::Mode;

// Move the picked discard-pile card to hand at cost 0 this turn; full hand leaves it
pub fn process_effect_liquid_memories_pick(id_target: Option<usize>, state: &mut GameState) {
    let Mode::Combat {
        id_hand,
        id_pile_discard,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_liquid_memories_pick outside Combat mode")
    };
    let id_target = id_target.expect("LiquidMemoriesPick requires id_target");
    if id_hand.len() >= MAX_SIZE_HAND {
        return;
    }
    if let Some(pos) = id_pile_discard.iter().position(|&v| v == id_target) {
        id_pile_discard.remove(pos);
    }
    id_hand.push(id_target);
    state.entities[id_target].card_cost_override = Some(CostOverride {
        amount: 0,
        scope: CostScope::Turn,
    });
}
