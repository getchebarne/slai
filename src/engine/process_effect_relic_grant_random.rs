use rand::Rng;

use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::pick_relic_by_roll;

// Rolls the name; the grant itself (ownership + on-pickup) is RelicGrantSpecific's job
pub fn process_effect_relic_grant_random(state: &mut GameState) {
    let roll = state.rng.random_range(0..100) as u8;
    let name = pick_relic_by_roll(
        roll,
        RELIC_TIER_TH_COMMON,
        RELIC_TIER_TH_UNCOMMON,
        &state.id_relics,
        &mut state.rng,
    );
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
