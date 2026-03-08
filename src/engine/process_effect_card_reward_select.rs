use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_select(
    card_idx: usize,
    card_rewards: &mut Vec<EntityId>,
    deck: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    // Get card id
    let card_id = card_rewards[card_idx];
    deck.push(card_id);

    // Queue top-effect to clear the queue
    ProcessEffectResult::Continue {
        top: vec![Effect {
            kind: EffectKind::CardRewardClear,
            source: None,
            target: None,
        }],
        bot: Vec::new(),
    }
}
