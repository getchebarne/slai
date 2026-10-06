use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Mayhem: the stack's target is fixed before the draw; its draw-top play queues behind the draw
pub fn process_effect_mayhem_proc(id_target: Option<usize>, state: &mut GameState) {
    let id_monster = id_target.expect("MayhemProc requires id_target");
    state.effect_queue.push_back(Effect {
        kind: EffectKind::CardPlayFromDrawTop,
        id_source: None,
        target: Target::Direct(Some(id_monster)),
    });
}
