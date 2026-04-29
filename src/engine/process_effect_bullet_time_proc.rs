use crate::engine::DispatchResult;
use crate::entity::Entity;

pub fn process_effect_bullet_time_proc(
    entities: &mut [Entity],
    id_hand: &[usize],
) -> DispatchResult {
    for &id_card in id_hand {
        entities[id_card].card_cost_override = Some(0);
    }
    DispatchResult::Continue
}
