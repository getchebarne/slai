use crate::entity::Entity;
use crate::modifier::modifier_set_not_new;
use crate::types::Phase;

pub fn process_effect_modifier_set_not_new(
    id_character: usize,
    entities: &mut [Entity],
    id_alive_monsters: &[usize],
) -> Option<Phase> {
    modifier_set_not_new(&mut entities[id_character].modifiers);
    for &id_monster in id_alive_monsters {
        modifier_set_not_new(&mut entities[id_monster].modifiers);
    }
    None
}
