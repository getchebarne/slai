use crate::consts::MAX_SIZE_CARD_PILE_HAND;
use crate::effect::EFFECT_ACCURACY_RESYNC_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::CardPile;
use crate::types::CostScope;
use crate::utils::detach_card;
use crate::utils::place_card;

pub fn process_effect_card_move(
    id_targets: &[usize],
    state: &mut GameState,
    pile: CardPile,
    cost_zero: Option<CostScope>,
) {
    // No on-discard or on-draw Card hooks fire
    assert!(!id_targets.is_empty(), "CardMove requires id_target");
    for &id_target in id_targets {
        // A full hand turns a Card away to the discard pile, so one already there keeps its place
        if pile == CardPile::Hand
            && state.combat.id_card_pile_hand.len() >= MAX_SIZE_CARD_PILE_HAND
            && state.combat.id_card_pile_discard.contains(&id_target)
        {
            continue;
        }

        // Remove from current pile
        detach_card(&mut state.combat, id_target);

        // Place in new one
        let placed = place_card(state, id_target, pile);

        // A Card moved into the draw or discard pile costs its combat cost again this turn
        if matches!(pile, CardPile::Draw | CardPile::Discard) || !placed {
            state.entities[id_target].card_cost_override = None;
        }

        // Only a placed Card takes the cost break; one a full hand reroutes to discard takes none
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

        // Accuracy: a Card landing in the discard pile resets every Shiv in the hand
        if (pile == CardPile::Discard || !placed)
            && has_modifier(
                &state.entities[state.id_character].modifiers,
                ModifierKind::Accuracy,
            )
        {
            state.effect_queue.push_front(EFFECT_ACCURACY_RESYNC_HAND);
        }
    }
}
