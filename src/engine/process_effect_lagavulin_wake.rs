use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::modifier::ModifierKind;

// The Lagavulin comes out of its shell: its Metallicize goes behind everything queued
pub fn process_effect_lagavulin_wake(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("LagavulinWake requires id_target");
    state.effect_queue.push_back(Effect {
        kind: EffectKind::ModifierRemove {
            kind: ModifierKind::Metallicize,
        },
        id_source: None,
        target: Target::Direct(Some(id_target)),
    });
}
