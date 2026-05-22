use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Gain `energy` energy if at least one card has been explicitly discarded
// this turn. The counter is increased in `process_effect_card_discard`
pub fn process_effect_sneaky_strike_proc(state: &mut GameState, energy: u8) {
    if state.this_turn_discards == 0 {
        return;
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::EnergyGain {
            amount: energy as u16,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
