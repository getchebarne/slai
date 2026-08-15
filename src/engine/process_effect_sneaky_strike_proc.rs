use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::Combat;
use crate::types::DeltaSign;

// Gain `energy` if any explicit discard this turn
pub fn process_effect_sneaky_strike_proc(state: &mut GameState, energy: u8) {
    assert!(
        state.combat.active,
        "process_effect_sneaky_strike_proc outside the Combat frame"
    );
    let Combat {
        this_turn_discards, ..
    } = &mut state.combat;
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
