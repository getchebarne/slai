use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::Combat;
use crate::types::CostScope;
use crate::utils::push_entity;

pub fn process_effect_card_nightmare_pick(id_target: Option<usize>, state: &mut GameState) {
    assert!(
        state.combat.active,
        "process_effect_card_nightmare_pick outside the Combat frame"
    );
    let Combat {
        id_card_nightmares, ..
    } = &mut state.combat;
    let id_target = id_target.expect("CardNightmarePick requires id_target");
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
    let id = push_entity(&mut state.entities, card);
    id_card_nightmares.push(id);
}
