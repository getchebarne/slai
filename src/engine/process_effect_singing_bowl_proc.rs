use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::first_target;
use crate::game::GameState;
use crate::types::DeltaSign;

pub fn process_effect_singing_bowl_proc(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
) {
    let id_target = first_target(state, EffectKind::SingingBowlProc, id_source, id_targets);
    let id_card = id_target.expect("SingingBowlProc requires id_target");
    assert!(
        state.reward.active,
        "SingingBowlProc outside the Reward context"
    );

    // Remove bundle
    let idx = state
        .reward
        .id_cards
        .iter()
        .position(|bundle| bundle.contains(&id_card))
        .expect("Forfeited Card is in a staged bundle");
    state.reward.id_cards.remove(idx);

    // A selection screen closes on Singing Bowl as on a take
    if state.reward.closes_on_selection {
        state.reward.active = false;
    }

    // Push effect for max health gain
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(2),
        },
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });
}
