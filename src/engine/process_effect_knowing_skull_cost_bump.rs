use crate::effect::Amount;
use crate::effect::EffectKind;
use crate::game::GameState;
use crate::types::Focus;
use crate::utils::context_focus;

// Escalate the selected wish's baked HP cost by one (the reference's ++cost);
// mutating baked option effects mirrors how combat mutates card effects
pub fn process_effect_knowing_skull_cost_bump(id_source: Option<usize>, state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::Event,
        "KnowingSkullCostBump outside the Event context"
    );
    let id_option = id_source.expect("KnowingSkullCostBump is baked with its option as source");
    match &mut state.entities[id_option].event_option_effects[0].kind {
        EffectKind::HealthDelta {
            amount: Amount::Absolute(cost),
            ..
        } => *cost += 1,
        _ => unreachable!("Knowing Skull wish options lead with the baked HP cost"),
    }
}
