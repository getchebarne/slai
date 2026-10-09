use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::the_guardian;

// The Guardian switches to Defensive Mode: Defensive Frame is its next move at once; Mode Shift goes and the 20 block lands behind everything queued
pub fn process_effect_defensive_mode(id_target: Option<usize>, state: &mut GameState) {
    let id_guardian = id_target.expect("DefensiveMode requires id_target");
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MoveUpdate {
            move_override: Some(the_guardian::IDX_MOVE_DEFENSIVE_MODE),
        },
        id_source: None,
        target: Target::Direct(Some(id_guardian)),
    });
    state.effect_queue.push_back(Effect {
        kind: EffectKind::ModifierRemove {
            kind: ModifierKind::ModeShift,
        },
        id_source: None,
        target: Target::Direct(Some(id_guardian)),
    });
    state.effect_queue.push_back(Effect {
        kind: EffectKind::BlockGain {
            amount: the_guardian::DEFENSIVE_MODE_BLOCK,
        },
        id_source: Some(id_guardian),
        target: Target::Direct(Some(id_guardian)),
    });
}
