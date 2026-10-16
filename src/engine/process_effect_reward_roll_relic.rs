use rand::Rng;

use crate::effect::RelicExclusion;
use crate::effect::RelicRoll;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::RelicName;
use crate::types::reward_ensure;
use crate::utils::draw_relic_excluding;
use crate::utils::push_entity;
use crate::utils::relic_tier_by_roll;
use crate::utils::roll_relic_from_pool;

pub fn process_effect_reward_roll_relic(
    state: &mut GameState,
    roll: RelicRoll,
    exclusion: RelicExclusion,
) {
    reward_ensure(&mut state.reward);

    let name = match roll {
        RelicRoll::Thresholds {
            th_common,
            th_uncommon,
        } => {
            let tier =
                relic_tier_by_roll(state.rng.random_range(0..100) as u8, th_common, th_uncommon);
            draw_relic_excluding(state, tier, exclusion)
        }
        RelicRoll::Tier(tier) => draw_relic_excluding(state, tier, exclusion),
        RelicRoll::Pool(pool) => roll_relic_from_pool(pool, &state.id_relics, &mut state.rng)
            .unwrap_or(RelicName::Circlet),
        RelicRoll::Name(name) => name,
    };

    let id = push_entity(&mut state.entities, get_relic(name));
    state.reward.id_relics.push(id);
}
