use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;

// Weak on target -> +1 energy +1 card (fires even on dead targets)
pub fn process_effect_heel_hook_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("HeelHookProc requires id_target");
    if !modifier_has(&state.entities[id_target].modifiers, ModifierKind::Weak) {
        return;
    }
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardDraw { count: 1 },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::EnergyGain { amount: 1 },
        id_source: None,
        target: Target::Direct(None),
    });
}
