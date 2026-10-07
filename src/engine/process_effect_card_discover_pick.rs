use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CardPile;
use crate::types::Combat;
use crate::types::CostScope;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::push_entity;

pub fn process_effect_card_discover_pick(
    id_target: Option<usize>,
    state: &mut GameState,
    cost_zero: Option<CostScope>,
    pile: CardPile,
    copies: u8,
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

    // The pick and its copies enter the pile in order; Discovery (Card) grants cost 0 this turn, Toolbox (Relic) keeps the printed cost
    state.effect_buf.clear();
    if let Some(scope) = cost_zero {
        state.effect_buf.push(Effect {
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
    state.effect_buf.push(Effect {
        kind: EffectKind::CardPlace { pile },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });

    // Sacred Bark's second copy is a stat-equivalent clone, costed the same way
    for _ in 1..copies {
        let copy = state.entities[id_card];
        let id_copy = push_entity(&mut state.entities, copy);
        if let Some(scope) = cost_zero {
            state.effect_buf.push(Effect {
                kind: EffectKind::SetCostOverride {
                    amount: 0,
                    only_reduce: false,
                    random: false,
                    scope,
                },
                id_source: None,
                target: Target::Direct(Some(id_copy)),
            });
        }
        state.effect_buf.push(Effect {
            kind: EffectKind::CardPlace { pile },
            id_source: None,
            target: Target::Direct(Some(id_copy)),
        });
    }
    flush_effects_from_buf_to_queue_front(state);
}
