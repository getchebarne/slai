use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::types::Mode;
use crate::utils::place_card;

pub fn process_effect_card_discover_pick(
    id_target: Option<usize>,
    state: &mut GameState,
    cost_zero: Option<CostScope>,
) {
    let Mode::Combat { id_discover, .. } = &mut state.mode else {
        unreachable!("process_effect_card_discover_pick outside Combat mode")
    };
    let id_card = id_target.expect("CardDiscoverPick Direct form must have target");
    id_discover.clear();

    // Discovery grants cost 0 this turn; Toolbox keeps the printed cost
    if let Some(scope) = cost_zero {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::SetCostOverride {
                amount: 0,
                only_reduce: false,
                scope,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    place_card(state, id_card, CardPile::Hand);
}
