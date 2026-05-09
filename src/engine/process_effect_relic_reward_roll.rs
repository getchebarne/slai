use rand::Rng;

use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::relics::get_relic;
use crate::types::RelicName;

pub fn process_effect_relic_reward_roll(
    id_relic_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    _rng: &mut impl Rng,
) -> DispatchResult {
    let id = entities.len();
    entities.push(get_relic(RelicName::SnakeRing));
    id_relic_rewards.push(id);
    DispatchResult::Continue
}
