use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_reward_take_gold(
    phase: &mut Phase,
    entities: &mut [Entity],
    id_character: usize,
) -> Option<Phase> {
    if let Phase::Reward { gold, .. } = phase {
        if let Some(amount) = gold.take() {
            entities[id_character].character_gold =
                entities[id_character].character_gold.saturating_add(amount);
        }
    }
    None
}
