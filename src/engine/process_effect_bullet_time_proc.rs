use crate::engine::DispatchResult;
use crate::entity::Entity;

// BulletTimeProc: set every card in hand to cost 0 for the rest of this turn
// by writing the per-instance `card_cost_override`. Reset to None happens at
// character TurnEnd (process_effect_turn_end_character) for all combat-instance
// cards, so newly-drawn cards next turn cost normal.
pub fn process_effect_bullet_time_proc(
    entities: &mut [Entity],
    id_hand: &[usize],
) -> DispatchResult {
    for &id_card in id_hand {
        entities[id_card].card_cost_override = Some(0);
    }
    DispatchResult::Continue
}
