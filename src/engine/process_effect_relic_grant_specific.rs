use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::get_relic;
use crate::types::RelicName;
use crate::utils::push_entity;

pub fn process_effect_relic_grant_specific(
    state: &mut GameState,
    name: RelicName,
    fallback_circlet: bool,
) {
    let owns_target = state.id_relics[name as usize].is_some();
    let target = match (owns_target, fallback_circlet) {
        (false, _) => name,
        (true, true) => RelicName::Circlet,
        (true, false) => return,
    };
    if state.id_relics[target as usize].is_some() {
        return;
    }
    let id = push_entity(&mut state.entities, get_relic(target));
    state.effect_queue.push_front(Effect {
        kind: EffectKind::RelicAdopt,
        id_source: None,
        target: Target::Direct(Some(id)),
    });
}
