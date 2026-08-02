use rand::Rng;

use crate::consts::RELIC_TIER_TH_COMMON;
use crate::consts::RELIC_TIER_TH_UNCOMMON;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::utils::pick_relic_by_roll;
use crate::utils::push_entity;

pub fn process_effect_relic_grant_random(state: &mut GameState) {
    let roll = state.rng.random_range(0..100) as u8;
    let name = pick_relic_by_roll(
        roll,
        RELIC_TIER_TH_COMMON,
        RELIC_TIER_TH_UNCOMMON,
        &state.id_relics,
        &mut state.rng,
    );
    // The roll excludes owned Relics but can fall back to an owned Circlet
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
