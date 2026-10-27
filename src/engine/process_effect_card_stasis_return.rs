use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::first_target;
use crate::entity::CardCostKind;
use crate::game::GameState;
use crate::types::CardPile;

// A Stasis hostage whose holder left goes back to the hand, or to the discard pile if the hand was full as its holder left
pub fn process_effect_card_stasis_return(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    hand_full: bool,
) {
    let id_target = first_target(
        state,
        EffectKind::CardStasisReturn { hand_full },
        id_source,
        id_targets,
    );
    assert!(
        state.combat.active,
        "process_effect_card_stasis_return outside the Combat frame"
    );
    let id_card = id_target.expect("CardStasisReturn requires id_target");

    let pile = if hand_full {
        // A hostage returned to the discard pile costs its combat cost this turn
        state.entities[id_card].card_cost_override = None;
        CardPile::Discard
    } else {
        // An Eviscerate returned to the hand restarts its cost this turn at its combat cost less this turn's discards; push_front reverses, so push it first
        if state.entities[id_card].card_cost_kind == CardCostKind::MinusDiscardsThisTurn {
            state.effect_queue.push_front(Effect {
                kind: EffectKind::CardCostMinusDiscards,
                id_source: None,
                target: Target::Direct(Some(id_card)),
            });
        }
        CardPile::Hand
    };
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardPlace { pile },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
