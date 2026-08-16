use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::utils::detach_card;
use crate::utils::place_card;

pub fn process_effect_card_move(
    id_target: Option<usize>,
    state: &mut GameState,
    pile: CardPile,
    cost_zero: Option<CostScope>,
) {
    // Relocation only — no discard / draw triggers fire
    let id_target = id_target.expect("CardMove requires id_target");

    // Remove from current pile
    detach_card(&mut state.combat, id_target);

    // Place in new one
    let placed = place_card(state, id_target, pile);

    // A full hand reroutes to discard without the cost break (source game parity)
    if placed && let Some(scope) = cost_zero {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::SetCostOverride {
                amount: 0,
                only_reduce: false,
                random: false,
                scope,
            },
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
}
