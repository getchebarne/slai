use rand::Rng;

use crate::effect::RelicExclusion;
use crate::effect::RelicPick;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::RelicName;
use crate::types::reward_ensure;
use crate::utils::draw_relic_excluding;
use crate::utils::pick_relic_from_pool;
use crate::utils::push_entity;
use crate::utils::relic_tier_by_roll;

pub fn process_effect_reward_roll_relic(
    state: &mut GameState,
    pick: RelicPick,
    exclusion: RelicExclusion,
) {
    reward_ensure(&mut state.reward);

    let name = match pick {
        RelicPick::Thresholds {
            th_common,
            th_uncommon,
        } => {
            let roll = state.rng.random_range(0..100) as u8;
            let tier = relic_tier_by_roll(roll, th_common, th_uncommon);
            draw_relic_excluding(state, tier, exclusion)
        }
        RelicPick::Tier(tier) => draw_relic_excluding(state, tier, exclusion),
        RelicPick::Pool(pool) => pick_relic_from_pool(pool, &state.id_relics, &mut state.rng)
            .unwrap_or(RelicName::Circlet),
        RelicPick::Name(name) => name,
    };

    let id = push_entity(&mut state.entities, get_relic(name));
    state.reward.id_relics.push(id);
}
