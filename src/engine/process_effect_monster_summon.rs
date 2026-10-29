use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Summon;
use crate::effect::Target;
use crate::game::GameState;
use rand::Rng;

pub fn process_effect_monster_summon(state: &mut GameState, summon: &Summon) {
    // A fight's first summon comes before any spawn opened the combat, so every post is free
    let combat = &state.combat;
    let post_free = |slot: usize| !combat.active || combat.id_monsters[slot].is_none();

    // The first free post takes the summon; with every post held it fizzles before the roll
    let Some(&slot) = summon.posts.iter().find(|&&slot| post_free(slot)) else {
        return;
    };

    // The spawn goes first, so a second summon in the same move finds this post taken
    state.effect_queue.push_front(Effect {
        kind: EffectKind::MonsterSpawn {
            name: summon.pool[state.rng.random_range(0..summon.pool.len())],
            minion: summon.minion,
            slot,
        },
        id_source: None,
        target: Target::Direct(None),
    });
}
