use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Frame;
use crate::utils::frame_top_mut;

// Gain `energy` if any explicit discard this turn
pub fn process_effect_sneaky_strike_proc(state: &mut GameState, energy: u8) {
    let Frame::Combat {
        this_turn_discards, ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("process_effect_sneaky_strike_proc outside the Combat frame")
    };
    if *this_turn_discards == 0 {
        return;
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: energy as u16,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
