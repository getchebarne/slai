use rand::Rng;
use strum::EnumCount;

use crate::engine::entities_push;
use crate::entity::Entity;
use crate::game::GameState;
use crate::relics::POOL_COMMON_RELIC;
use crate::relics::POOL_RARE_RELIC;
use crate::relics::POOL_UNCOMMON_RELIC;
use crate::relics::get_relic;
use crate::types::RelicName;
use crate::types::RelicTier;
use crate::utils::pick_from_pool;

// StS's `returnRandomRelicTier()`: Common 50%, Uncommon 33%, Rare 17%
const COMMON_PCT: u8 = 50;
const COMMON_PLUS_UNCOMMON_PCT: u8 = 83;

pub fn process_effect_relic_grant_random(state: &mut GameState, tier: Option<RelicTier>) {
    let rolled_tier = tier.unwrap_or_else(|| roll_tier(&mut state.rng));
    let name = match rolled_tier {
        RelicTier::Common => pick_from_pool(POOL_COMMON_RELIC, &state.id_relics, &mut state.rng)
            .or_else(|| pick_from_pool(POOL_UNCOMMON_RELIC, &state.id_relics, &mut state.rng))
            .or_else(|| pick_from_pool(POOL_RARE_RELIC, &state.id_relics, &mut state.rng))
            .unwrap_or(RelicName::Circlet),
        RelicTier::Uncommon => {
            pick_from_pool(POOL_UNCOMMON_RELIC, &state.id_relics, &mut state.rng)
                .or_else(|| pick_from_pool(POOL_RARE_RELIC, &state.id_relics, &mut state.rng))
                .unwrap_or(RelicName::Circlet)
        }
        RelicTier::Rare => pick_from_pool(POOL_RARE_RELIC, &state.id_relics, &mut state.rng)
            .unwrap_or(RelicName::Circlet),
        _ => RelicName::Circlet,
    };
    grant_relic(name, &mut state.id_relics, &mut state.entities);
}

fn roll_tier(rng: &mut impl Rng) -> RelicTier {
    let roll = rng.random_range(0..100) as u8;
    if roll < COMMON_PCT {
        RelicTier::Common
    } else if roll < COMMON_PLUS_UNCOMMON_PCT {
        RelicTier::Uncommon
    } else {
        RelicTier::Rare
    }
}

pub fn grant_relic(
    name: RelicName,
    id_relics: &mut [Option<usize>; RelicName::COUNT],
    entities: &mut Vec<Entity>,
) {
    let id = entities_push(entities, get_relic(name));
    id_relics[name as usize] = Some(id);
}
