use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::find_free_slot;
use crate::potions::get_potion;
use crate::potions::get_random_potion_name;
use crate::utils::push_entity;

pub fn process_effect_potion_add_random(state: &mut GameState, limited: bool) {
    if find_free_slot(&state.id_potions, state.potion_slots_max).is_none() {
        return;
    }
    let name = get_random_potion_name(&mut state.rng, limited);
    let id = push_entity(&mut state.entities, get_potion(name));
    state.effect_queue.push_front(Effect {
        kind: EffectKind::PotionAdopt,
        id_source: None,
        target: Target::Direct(Some(id)),
    });
}
