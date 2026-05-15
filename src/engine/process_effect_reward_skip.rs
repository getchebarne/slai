use crate::types::Phase;

pub fn process_effect_reward_skip(phase: &mut Phase) -> Option<Phase> {
    if let Phase::Reward { id_cards, id_relic, id_potion, gold } = phase {
        id_cards.clear();
        *id_relic = None;
        *id_potion = None;
        *gold = None;
    }
    None
}
