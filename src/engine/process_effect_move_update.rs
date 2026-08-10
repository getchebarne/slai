use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::effect::EffectKind;
use crate::entity::Intent;
use crate::entity::get_move_history_slice;
use crate::entity::push_move_history;
use crate::game::GameState;
use crate::monsters::book_of_stabbing;
use crate::monsters::get_next_move;
use crate::monsters::hexaghost;
use crate::monsters::is_cycle_boundary;
use crate::types::Mode;
use crate::types::MonsterName;
use crate::utils::mode_top_mut;

pub fn process_effect_move_update(
    id_target: Option<usize>,
    state: &mut GameState,
    move_override: Option<usize>,
) {
    let Mode::Combat { id_monsters, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("process_effect_move_update outside Combat mode")
    };
    let id_target = id_target.expect("MoveUpdate requires id_target");

    // Corpses don't roll: a mid-phase death leaves this queued effect dangling
    if state.entities[id_target].dead {
        return;
    }
    let character_health = state.entities[state.id_character].vitals.health;

    // A forced move (Split, wake-up) skips the AI and its RNG draw
    let move_next = match move_override {
        Some(idx) => idx,
        None => get_next_move(
            &state.entities,
            id_target,
            &id_monsters,
            state.ascension,
            &mut state.rng,
        ),
    };

    let entity = &mut state.entities[id_target];
    entity.monster_move_current = Some(move_next);

    // Divider damage locks in at selection; later HP changes don't move it
    if entity.monster_name == MonsterName::Hexaghost && move_next == hexaghost::IDX_MOVE_DIVIDER {
        let damage = character_health / 12 + 1;
        let move_divider = &mut entity.monster_moves[hexaghost::IDX_MOVE_DIVIDER];
        for effect in move_divider.effects[..move_divider.effects_len as usize].iter_mut() {
            if let EffectKind::DamagePhysical { amount } = &mut effect.kind {
                *amount = damage;
            }
        }
        move_divider.intent = Intent::Attack {
            damage,
            instances: HEXAGHOST_DIVIDER_HITS,
        };
    }

    // Multi-Stab hit count escalates over the fight; history excludes the current pick
    if entity.monster_name == MonsterName::BookOfStabbing
        && move_next == book_of_stabbing::IDX_MOVE_MULTI_STAB
    {
        let turns_taken = get_move_history_slice(entity).len();
        let move_stab = &mut entity.monster_moves[book_of_stabbing::IDX_MOVE_MULTI_STAB];
        let prev_len = move_stab.effects_len as usize;
        // The fixed-size effect array caps the escalation
        let hits = book_of_stabbing::multi_stab_hits(prev_len, turns_taken, state.ascension)
            .min(MAX_EFFECTS_PER_MOVE);
        let stab = move_stab.effects[0];
        let EffectKind::DamagePhysical { amount: damage } = stab.kind else {
            unreachable!("Multi-Stab leads with DamagePhysical")
        };
        for effect in move_stab.effects[prev_len..hits].iter_mut() {
            *effect = stab;
        }
        move_stab.effects_len = hits as u8;
        move_stab.intent = Intent::Attack {
            damage,
            instances: hits as u8,
        };
    }

    let move_idx = move_next as u8;
    push_move_history(entity, move_idx);

    if is_cycle_boundary(entity.monster_name, move_idx) {
        entity.monster_cycle_count += 1;
    }
}
