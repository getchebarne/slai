use crate::engine::process_effect_monster_spawn::process_effect_monster_spawn;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::pick_gremlin;

// Roll a gremlin from the weighted pool and spawn it as a Minion
pub fn process_effect_gremlin_summon(state: &mut GameState) {
    let name = pick_gremlin(&mut state.rng);
    if let Some(id_gremlin) = process_effect_monster_spawn(state, name) {
        modifier_apply(
            &mut state.entities[id_gremlin].modifiers,
            ModifierKind::Minion,
            1,
        );
    }
}
