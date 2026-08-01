use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
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
    let id_target = id_target.expect("LiquidMemories requires id_target");

    // If the hand is full, return without side effects
    if id_hand.len() >= MAX_SIZE_HAND {
        return;
    }

    // Remove target card from discard pile
    if let Some(pos) = id_pile_discard.iter().position(|&v| v == id_target) {
        id_pile_discard.remove(pos);
    }

    // Push it to the hand
    id_hand.push(id_target);

    // Costs 0 this turn
    state.effect_queue.push_front(Effect {
        kind: EffectKind::SetCostOverride {
            amount: 0,
            only_reduce: false,
            random: false,
            scope: CostScope::Turn,
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
}
