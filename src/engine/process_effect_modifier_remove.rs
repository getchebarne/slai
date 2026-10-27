use crate::effect::EffectKind;
use crate::engine::first_target;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_remove;

pub fn process_effect_modifier_remove(
    id_source: Option<usize>,
    id_targets: &[usize],
    state: &mut GameState,
    kind: ModifierKind,
) {
    let id_target = first_target(
        state,
        EffectKind::ModifierRemove { kind },
        id_source,
        id_targets,
    );
    let id_target = id_target.expect("ModifierRemove requires id_target");
    modifier_remove(&mut state.entities[id_target].modifiers, kind);
}
