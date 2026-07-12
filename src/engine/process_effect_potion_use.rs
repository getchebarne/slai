use crate::effect::Effect;
use crate::game::GameState;
use crate::potions::remove_potion;

pub fn process_effect_potion_use(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("PotionUse requires id_target");
    // Consume the potion from its belt slot before its effects run
    remove_potion(&mut state.id_potions, id_potion);
    let potion = &state.entities[id_potion];
    for effect in potion.potion_effects.iter().rev() {
        state.effect_queue.push_front(Effect {
            id_source: Some(id_potion),
            ..*effect
        });
    }
}
