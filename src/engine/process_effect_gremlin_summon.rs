use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::pick_gremlin;

pub fn process_effect_gremlin_summon(state: &mut GameState) {
    // A full roster fizzles the summon before the pool roll; the first summon
    // of an encounter runs before any spawn opened the combat
    if state.combat.active && state.combat.id_monsters.iter().all(|slot| slot.is_some()) {
        return;
    }

    // Roll a gremlin from the weighted pool; the spawn stamps it Minion
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MonsterSpawn {
            name: pick_gremlin(&mut state.rng),
            minion: true,
            cap: None,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
