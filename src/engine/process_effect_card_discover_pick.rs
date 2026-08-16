use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardPile;
use crate::types::Combat;
use crate::types::CostScope;
use crate::utils::place_card;

pub fn process_effect_card_discover_pick(
    id_target: Option<usize>,
    state: &mut GameState,
    cost_zero: Option<CostScope>,
    pile: CardPile,
) {
    assert!(
        state.combat.active,
        "process_effect_card_discover_pick outside the Combat frame"
    );
    let Combat {
        id_card_discover, ..
    } = &mut state.combat;
    let id_card = id_target.expect("CardDiscoverPick Direct form must have target");

    // Clear discovered Cards
    id_card_discover.clear();

    // Discovery (Card) grants cost 0 this turn; Toolbox (Relic) keeps the printed cost
    if let Some(scope) = cost_zero {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::SetCostOverride {
                amount: 0,
                only_reduce: false,
                random: false,
                scope,
            },
            id_source: None,
            target: Target::Direct(Some(id_card)),
        });
    }
    place_card(state, id_card, pile);
}
