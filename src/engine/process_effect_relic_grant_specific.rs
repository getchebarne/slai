use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::RelicName;
use crate::utils::push_entity;

pub fn process_effect_relic_grant_specific(state: &mut GameState, name: RelicName) {
    // Push
    let id_relic = push_entity(&mut state.entities, get_relic(name));
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicAdopt,
        id_source: None,
        target: Target::Direct(Some(id_relic)),
    });
}
