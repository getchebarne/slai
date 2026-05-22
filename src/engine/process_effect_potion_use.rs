use crate::effect::Effect;
use crate::game::GameState;

pub fn process_effect_potion_use(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("PotionUse requires id_target");
    let potion = &state.entities[id_potion];
    for effect in potion.potion_effects.iter().rev() {
        state.effect_queue.push_front(Effect {
            id_source: Some(id_potion),
            ..*effect
        });
    }
}
