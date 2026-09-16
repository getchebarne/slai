use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::belt_has_room;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::potions::get_random_potion_name_uniform;
use crate::utils::push_entity;

pub fn process_effect_potion_add_random(state: &mut GameState, limited: bool, uniform: bool) {
    if !belt_has_room(&state.id_potions, state.potion_slots_max) {
        return;
    }
    // The Fruit Juice ban only applies in combat (Entropic Brew outside it draws freely)
    let limited = limited && state.combat.active;
    let name = if uniform {
        get_random_potion_name_uniform(&mut state.rng)
    } else {
        get_random_potion_name(&mut state.rng, limited)
    };
    let id = push_entity(&mut state.entities, get_potion(name));
    state.effect_queue.push_front(Effect {
        kind: EffectKind::PotionAdopt,
        id_source: None,
        target: Target::Direct(Some(id)),
    });
}
