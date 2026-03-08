use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_select(
    id_card: usize,
    card_rewards: &mut Vec<EntityId>,
    deck: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    // Get card id
    let id_card = card_rewards[id_card];
    deck.push(id_card);

    // Queue top-effect to clear the queue
    ProcessEffectResult::AddAndContinue {
        top: vec![Effect {
            kind: EffectKind::CardRewardClear,
            source: None,
            target: None,
        }],
        bot: Vec::new(),
    }
}
