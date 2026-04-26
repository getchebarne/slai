use crate::engine::DispatchResult;
use crate::entity::Entity;

// Nightmare direct-form: snapshot the picked card `count` times into
// `cards_nightmare`. The character TurnStart spawn hook drains the vec on
// the next player turn. Card stays in hand (matches StS — NightmareAction
// removes via the picker UI then re-adds, net no change).
pub fn process_effect_card_nightmare_pick(
    entities: &[Entity],
    id_card: usize,
    count: u8,
    cards_nightmare: &mut Vec<Entity>,
) -> DispatchResult {
    let template = entities[id_card];
    for _ in 0..count {
        cards_nightmare.push(template);
    }
    DispatchResult::Continue
}
