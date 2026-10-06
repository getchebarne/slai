use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::gremlin_leader::GREMLIN_POSTS;
use crate::monsters::pick_gremlin;

pub fn process_effect_gremlin_summon(state: &mut GameState) {
    // The first post no living Monster holds; with all three held the summon fizzles before
    // the pool roll. The first summon of an encounter runs before any spawn opened the combat
    let combat = &state.combat;
    let post_free = |x: i16| {
        !combat.active
            || !combat
                .id_monsters
                .iter()
                .flatten()
                .any(|&id| state.entities[id].monster_x == x)
    };
    let Some(&x) = GREMLIN_POSTS.iter().find(|&&x| post_free(x)) else {
        return;
    };

    // Roll a gremlin from the weighted pool; the spawn stamps it Minion
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MonsterSpawn {
            name: pick_gremlin(&mut state.rng),
            minion: true,
            x,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
