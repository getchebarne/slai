use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_has;

// If the targeted enemy has Weak, gain 1 energy and draw 1 card. Weak
// still reads on dead entities so the bonus fires even if the target
// died from the preceding hit
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
