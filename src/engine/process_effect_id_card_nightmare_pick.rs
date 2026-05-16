use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_id_card_nightmare_pick(
    entities: &mut Vec<Entity>,
    id_target: usize,
    id_card_nightmare: &mut Option<usize>,
) -> Option<Phase> {
    let card = entities[id_target];
    let id = entities.len();
    entities.push(card);
    *id_card_nightmare = Some(id);
    None
}
