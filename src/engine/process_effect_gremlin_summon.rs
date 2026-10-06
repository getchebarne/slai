use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::monsters::gremlin_leader::GREMLIN_POSTS;
use crate::monsters::pick_gremlin;

pub fn process_effect_gremlin_summon(state: &mut GameState) {
    // The first post whose slot is empty; with all three held the summon fizzles before the
    // pool roll. The first summon of an encounter runs before any spawn opened the combat
    let combat = &state.combat;
    let post_free = |slot: usize| !combat.active || combat.id_monsters[slot].is_none();
    let Some(&slot) = GREMLIN_POSTS.iter().find(|&&slot| post_free(slot)) else {
        return;
    };

    // Roll a gremlin from the weighted pool; the spawn stamps it Minion
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MonsterSpawn {
            name: pick_gremlin(&mut state.rng),
            minion: true,
            slot,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
