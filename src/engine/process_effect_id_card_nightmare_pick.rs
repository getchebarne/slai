use crate::engine::DispatchResult;
use crate::entity::Entity;

pub fn process_effect_id_card_nightmare_pick(
    entities: &mut Vec<Entity>,
    id_target: usize,
    id_card_nightmare: &mut Option<usize>,
) -> DispatchResult {
    let card = entities[id_target];
    let id = entities.len();
    entities.push(card);
    *id_card_nightmare = Some(id);
    DispatchResult::Continue
}
