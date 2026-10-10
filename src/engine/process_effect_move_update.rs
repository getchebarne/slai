use crate::consts::HEXAGHOST_DIVIDER_DIVISOR;
use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::effect::EffectKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::book_of_stabbing;
use crate::monsters::get_next_move;
use crate::monsters::hexaghost;
use crate::monsters::is_cycle_boundary;
use crate::monsters::push_move_history;
use crate::monsters::the_guardian;
use crate::types::Combat;
use crate::types::MonsterName;

pub fn process_effect_move_update(
    id_target: Option<usize>,
    state: &mut GameState,
    move_override: Option<usize>,
) {
    assert!(
        state.combat.active,
        "process_effect_move_update outside the Combat frame"
    );
    let Combat { id_monsters, .. } = &mut state.combat;
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

    // The Monster keeps its own copy of the chosen move's effects
    let move_chosen = entity.monster_moves[move_next];
    entity.monster_move_effects = move_chosen.effects;
    entity.monster_move_effects_len = move_chosen.effects_len;

    // Divider's damage locks in at selection; later HP changes don't move it
    let damage_locked = (entity.monster_name == MonsterName::Hexaghost
        && move_next == hexaghost::IDX_MOVE_DIVIDER)
        .then(|| character_health / HEXAGHOST_DIVIDER_DIVISOR + 1);

    // Multi-Stab's hit count locks in at selection, from the moves chosen before it
    let instances_locked = (entity.monster_name == MonsterName::BookOfStabbing
        && move_next == book_of_stabbing::IDX_MOVE_MULTI_STAB)
        .then(|| {
            book_of_stabbing::multi_stab_instances(&entity.monster_move_uses, state.ascension)
        });

    // The cycle count already includes the move being chosen
    let move_idx = move_next as u8;
    if is_cycle_boundary(entity.monster_name, move_idx) {
        entity.monster_cycle_count += 1;
    }

    // Twin Slam's Mode Shift locks in at selection: 10 more for each Defensive Mode before it
    let mode_shift_bonus = if entity.monster_name == MonsterName::TheGuardian
        && move_next == the_guardian::IDX_MOVE_TWIN_SLAM
    {
        MODE_SHIFT_INCREASE_PER_CYCLE * i16::from(entity.monster_cycle_count)
    } else {
        0
    };

    // The locked values go into the copy
    for effect in entity.monster_move_effects[..move_chosen.effects_len as usize].iter_mut() {
        match &mut effect.kind {
            EffectKind::DamagePhysical {
                amount, instances, ..
            } => {
                *amount = damage_locked.unwrap_or(*amount);
                *instances = instances_locked.map_or(*instances, u16::from);
            }
            EffectKind::ModifierDelta {
                kind: ModifierKind::ModeShift,
                stacks,
            } => *stacks += mode_shift_bonus,
            _ => {}
        }
    }

    push_move_history(entity, move_idx);
}
