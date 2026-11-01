use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::lagavulin;

// The Lagavulin comes out of its shell: it sheds its sleep kit's 8 Metallicize behind everything queued
pub fn process_effect_lagavulin_wake(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("LagavulinWake requires id_target");
    state.effect_queue.push_back(Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Metallicize,
            stacks: -lagavulin::METALLICIZE_SLEEP,
        },
        id_source: Some(id_target),
        target: Target::Direct(Some(id_target)),
    });
}
