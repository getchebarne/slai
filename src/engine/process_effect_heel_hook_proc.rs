use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::has_modifier;
use crate::types::DeltaSign;

// Weak on target -> +1 energy +1 card (fires even on dead targets)
pub fn process_effect_heel_hook_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("HeelHookProc requires id_target");
    if !has_modifier(&state.entities[id_target].modifiers, ModifierKind::Weak) {
        return;
    }
    // Executes in reverse:
    //     1. EnergyDelta
    //     2. CardDraw
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardDraw { count: 1 },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::EnergyDelta {
            sign: DeltaSign::Gain,
            amount: 1,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
