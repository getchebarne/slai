use crate::cards::get_card_template;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::CardCostKind;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::Combat;
use crate::types::CostScope;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_select(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_nightmare_select outside the Combat frame"
    );
    let Combat {
        id_card_pile_hand,
        id_card_pile_nightmare,
        modifier_seq_next,
        ..
    } = &mut state.combat;
    let id_target = id_target.expect("CardNightmareSelect requires id_target");
    let mut card = state.entities[id_target];

    // Reset attributes on the copy: the per-turn cut is dropped, free-to-play-once survives
    if matches!(
        card.card_cost_override,
        Some(CostOverride {
            scope: CostScope::Turn,
            ..
        })
    ) {
        card.card_cost_override = None;
    }

    // The copy exhausts as its printed Card does, whatever a Relic made of the original
    card.card_exhaust = get_card_template(card.card_name, card.card_upgraded).exhaust;
    let id = push_entity(&mut state.entities, card);

    // Each Nightmare takes its own stamp, which places its copies among the turn-start hooks
    id_card_pile_nightmare.push((id, *modifier_seq_next));
    *modifier_seq_next += 1;

    // The selected Card rejoins the hand at the end
    if let Some(pos) = id_card_pile_hand
        .iter()
        .position(|&id_card| id_card == id_target)
    {
        id_card_pile_hand.remove(pos);
        id_card_pile_hand.push(id_target);
    }

    // Copying an Eviscerate restarts its cost this turn at its combat cost less this turn's discards
    if state.entities[id_target].card_cost_kind == CardCostKind::MinusDiscardsThisTurn {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardCostMinusDiscards,
            id_source: None,
            target: Target::Direct(Some(id_target)),
        });
    }
}
