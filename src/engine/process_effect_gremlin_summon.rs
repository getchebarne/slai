use crate::engine::process_effect_monster_spawn::process_effect_monster_spawn;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::pick_gremlin;
use crate::types::Mode;
use crate::utils::mode_top;

// Roll a gremlin from the weighted pool and spawn it as a Minion
pub fn process_effect_gremlin_summon(state: &mut GameState) {
    // A full roster fizzles the summon
    if let Mode::Combat { id_monsters, .. } = mode_top(&state.mode_stack)
        && id_monsters.iter().all(|s| s.is_some())
    {
        return;
    }

    let name = pick_gremlin(&mut state.rng);
    process_effect_monster_spawn(state, name);

    // The spawned entity is the arena's newest
    let id_gremlin = state.entities.len() - 1;
    modifier_apply(
        &mut state.entities[id_gremlin].modifiers,
        ModifierKind::Minion,
        1,
    );
}
