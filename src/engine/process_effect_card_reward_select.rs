use crate::cards::Card;
use crate::engine::ProcessEffectResult;

pub fn process_effect_card_reward_select(
    card_idx: usize,
    card_rewards: &mut Vec<Card>,
    deck: &mut Vec<Card>,
) -> ProcessEffectResult {
    let card = card_rewards[card_idx];
    deck.push(card);
    card_rewards.clear();
    ProcessEffectResult::Pass
}
