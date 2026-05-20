use crate::consts::MAX_SIZE_HAND;
use crate::consts::NIGHTMARE_COPIES;
use crate::entity::Entity;

pub fn process_effect_id_card_nightmare_spawn(
    entities: &mut Vec<Entity>,
    id_hand: &mut Vec<usize>,
    id_pile_discard: &mut Vec<usize>,
    id_card_nightmare: &mut Option<usize>,
) {
    let id_template = id_card_nightmare
        .take()
        .expect("CardNightmareSpawn with no pending snapshot");

    let template = entities[id_template];
    for _ in 0..NIGHTMARE_COPIES {
        let id_card = entities.len();
        entities.push(template);
        if id_hand.len() < MAX_SIZE_HAND {
            id_hand.push(id_card);
        } else {
            id_pile_discard.push(id_card);
        }
    }
}
