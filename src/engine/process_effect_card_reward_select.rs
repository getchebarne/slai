use crate::effect::{Effect, EffectKind};
use crate::engine::ProcessEffectResult;
use crate::types::EntityId;

pub fn process_effect_card_reward_select(
    idx_reward: usize,
    card_rewards: &mut Vec<EntityId>,
    deck: &mut Vec<EntityId>,
) -> ProcessEffectResult {
    // Add selected card to deck
    let id_card = card_rewards[idx_reward];
    deck.push(id_card);

    // Queue top-effect to clear the rewards. The clear handler halts on
    // AwaitMapNode after clearing, which is the post-reward transition.
    ProcessEffectResult::AddAndContinue {
        top: vec![Effect {
            kind: EffectKind::CardRewardClear,
            source: None,
            target: None,
        }],
        bot: Vec::new(),
    }
}
