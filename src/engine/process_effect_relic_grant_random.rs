use rand::Rng;

use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::game::GameState;
use crate::utils::grant_relic;
use crate::utils::pick_relic_by_roll;

pub fn process_effect_relic_grant_random(state: &mut GameState) {
    let roll = state.rng.random_range(0..100) as u8;
    let name = pick_relic_by_roll(
        roll,
        RELIC_TIER_TH_COMMON,
        RELIC_TIER_TH_UNCOMMON,
        &state.id_relics,
        &mut state.rng,
    );
    grant_relic(name, &mut state.id_relics, &mut state.entities);
}
