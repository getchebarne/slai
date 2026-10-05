use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// The second play goes behind everything the first play's effects queued
pub fn process_effect_card_replay(id_target: Option<usize>, state: &mut GameState, energy: u16) {
    state.effect_queue.push_back(Effect {
        kind: EffectKind::CardPlay {
            replay: true,
            energy,
        },
        id_source: None,
        target: Target::Direct(id_target),
    });
}
