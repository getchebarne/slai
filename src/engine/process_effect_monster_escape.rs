use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;

// Mark dead WITHOUT firing the on-death hook chain
pub fn process_effect_monster_escape(id_target: Option<usize>, state: &mut GameState) {
    let id_target = id_target.expect("MonsterEscape requires id_target");
    state.entities[id_target].dead = true;
    if let Some(slot) = state.id_monsters.iter().position(|s| *s == Some(id_target)) {
        state.id_monsters[slot] = None;
    }
    state.this_combat_escaped = true;

    let any_alive = state.id_monsters.iter().any(|s| s.is_some());
    if !any_alive {
        state.effect_queue.clear();
        state.effect_queue.push_back(Effect {
            kind: EffectKind::CombatEnd,
            id_source: None,
            target: Target::Direct(None),
        });
    }
}
