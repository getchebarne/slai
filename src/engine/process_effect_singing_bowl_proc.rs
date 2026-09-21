use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;

pub fn process_effect_singing_bowl_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_card = id_target.expect("SingingBowlProc requires id_target");
    assert!(
        state.reward.active,
        "SingingBowlProc outside the Reward context"
    );

    // Forfeit the bundle the picked Card belongs to
    let idx = state
        .reward
        .id_cards
        .iter()
        .position(|bundle| bundle.contains(&id_card))
        .expect("SingingBowlProc names a staged Card");
    state.reward.id_cards.remove(idx);

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
