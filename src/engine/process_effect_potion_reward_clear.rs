use crate::types::Phase;

pub fn process_effect_potion_reward_clear(phase: &mut Phase) -> Option<Phase> {
    if let Phase::Reward { id_potion, .. } = phase {
        *id_potion = None;
    }
    None
}
