use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::RelicName;
use crate::utils::pick_relic_from_pool;

// Grant a uniformly-rolled unowned Relic from a fixed pool; Circlet when all are owned
pub fn process_effect_relic_grant_pool(state: &mut GameState, pool: &'static [RelicName]) {
    let relic_name =
        pick_relic_from_pool(pool, &state.id_relics, &mut state.rng).unwrap_or(RelicName::Circlet);

    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: relic_name,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
