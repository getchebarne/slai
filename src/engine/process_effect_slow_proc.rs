use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;

// Slow counts a Card play: its holder gives itself a stack behind everything queued, so Sadistic Nature stays silent
pub fn process_effect_slow_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("SlowProc requires id_target");
    state.effect_queue.push_back(Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Slow,
            stacks: 1,
        },
        id_source: Some(id_target),
        target: Target::Direct(Some(id_target)),
    });
}
