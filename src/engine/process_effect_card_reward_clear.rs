use crate::types::Phase;

pub fn process_effect_card_reward_clear(phase: &mut Phase) -> Option<Phase> {
    if let Phase::Reward { id_cards, .. } = phase {
        id_cards.clear();
    }
    None
}
