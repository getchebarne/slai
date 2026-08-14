use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Frame;
use crate::utils::frame_top_mut;

pub fn process_effect_singing_bowl_proc(state: &mut GameState, idx_bundle: u8) {
    let Frame::Reward {
        id_cards: bundles, ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("SingingBowlProc outside the Reward frame")
    };

    // Remove bundle
    bundles.remove(idx_bundle as usize);

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
