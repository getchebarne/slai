use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::RelicName;
use crate::utils::pick_relic_from_pool;
use crate::utils::push_entity;

// Grant a uniformly-rolled unowned Relic from a fixed pool; Circlet when all are owned
pub fn process_effect_relic_grant_pool(state: &mut GameState, pool: &'static [RelicName]) {
    let relic_name =
        pick_relic_from_pool(pool, &state.id_relics, &mut state.rng).unwrap_or(RelicName::Circlet);

    // The pick is unowned, or a Circlet the adopt counts on one already held
    let id_relic = push_entity(&mut state.entities, get_relic(relic_name));
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicAdopt,
        id_source: None,
        target: Target::Direct(Some(id_relic)),
    });
}
