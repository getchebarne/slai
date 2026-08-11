use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::pick_gremlin;
use crate::types::Mode;
use crate::utils::mode_top;

pub fn process_effect_gremlin_summon(state: &mut GameState) {
    // A full roster fizzles the summon before the pool roll
    if let Mode::Combat { id_monsters, .. } = mode_top(&state.mode_stack)
        && id_monsters.iter().all(|s| s.is_some())
    {
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
