use crate::types::Phase;

pub fn process_effect_relic_reward_clear(phase: &mut Phase) -> Option<Phase> {
    if let Phase::Reward { id_relic, .. } = phase {
        *id_relic = None;
    }
    None
}
