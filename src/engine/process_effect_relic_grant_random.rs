use rand::Rng;

use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::RelicExclusion;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::RelicTier;
use crate::utils::draw_relic_excluding;
use crate::utils::push_entity;
use crate::utils::relic_tier_by_roll;

pub fn process_effect_relic_grant_random(
    state: &mut GameState,
    tier: Option<RelicTier>,
    exclusion: RelicExclusion,
) {
    let tier = tier.unwrap_or_else(|| {
        relic_tier_by_roll(
            state.rng.random_range(0..100) as u8,
            RELIC_TIER_TH_COMMON,
            RELIC_TIER_TH_UNCOMMON,
        )
    });
    let name = draw_relic_excluding(state, tier, exclusion);
    // Exhausted pools fall back to a possibly owned Circlet
    if state.id_relics[name as usize].is_some() {
        return;
    }
    let id = push_entity(&mut state.entities, get_relic(name));
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicAdopt,
        id_source: None,
        target: Target::Direct(Some(id)),
    });
}
