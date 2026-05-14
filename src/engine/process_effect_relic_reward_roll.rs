use rand::Rng;
use strum::EnumCount;

use crate::engine::DispatchResult;
use crate::entity::Entity;
use crate::types::RelicName;
use crate::utils::add_relic_reward_for_roll;

pub fn process_effect_relic_reward_roll(
    th_common: u8,
    th_uncommon: u8,
    id_relics: &[Option<usize>; RelicName::COUNT],
    id_relic_rewards: &mut Vec<usize>,
    entities: &mut Vec<Entity>,
    rng: &mut impl Rng,
) -> DispatchResult {
    let roll = rng.random_range(0..100) as u8;
    add_relic_reward_for_roll(roll, th_common, th_uncommon, id_relics, id_relic_rewards, entities, rng);
    DispatchResult::Continue
}
