use crate::consts::HEXAGHOST_DIVIDER_DIVISOR;
use crate::consts::IT_IS_TIME_INCREASE_MAX;
use crate::consts::IT_IS_TIME_INCREASE_PER_USE;
use crate::consts::MODE_SHIFT_INCREASE_PER_CYCLE;
use crate::effect::EffectKind;
use crate::game::GameState;
use crate::modifier::ModifierKind;
use crate::monsters::book_of_stabbing;
use crate::monsters::get_next_move;
use crate::monsters::giant_head;
use crate::monsters::hexaghost;
use crate::monsters::maw;
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
            state.id_character,
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

    // Multi-Stab's and Nom's hit counts lock in at selection, from the moves used before them
    let instances_locked = match (entity.monster_name, move_next) {
        (MonsterName::BookOfStabbing, book_of_stabbing::IDX_MOVE_MULTI_STAB) => Some(
            book_of_stabbing::multi_stab_instances(&entity.monster_move_uses, state.ascension),
        ),
        (MonsterName::Maw, maw::IDX_MOVE_NOM) => {
            Some(maw::nom_instances(&entity.monster_move_uses))
        }
        _ => None,
    };

    // Twin Slam's Mode Shift locks in at selection: 10 more per Defensive Mode so far, each ending in one Twin Slam, this one included
    let mode_shift_bonus = if entity.monster_name == MonsterName::TheGuardian
        && move_next == the_guardian::IDX_MOVE_TWIN_SLAM
    {
        MODE_SHIFT_INCREASE_PER_CYCLE
            * (i16::from(entity.monster_move_uses[the_guardian::IDX_MOVE_TWIN_SLAM]) + 1)
    } else {
        0
    };

    // It Is Time's damage locks in at selection: 5 more per It Is Time used before it, 30 more at most
    let damage_bonus = if entity.monster_name == MonsterName::GiantHead
        && move_next == giant_head::IDX_MOVE_IT_IS_TIME
    {
        (IT_IS_TIME_INCREASE_PER_USE
            * u16::from(entity.monster_move_uses[giant_head::IDX_MOVE_IT_IS_TIME]))
        .min(IT_IS_TIME_INCREASE_MAX)
    } else {
        0
    };

    // The locked values go into the copy
    for effect in entity.monster_move_effects[..move_chosen.effects_len as usize].iter_mut() {
        match &mut effect.kind {
            EffectKind::DamagePhysical {
                amount, instances, ..
            } => {
                *amount = damage_locked.unwrap_or(*amount) + damage_bonus;
                *instances = instances_locked.map_or(*instances, u16::from);
            }
            EffectKind::ModifierDelta {
                kind: ModifierKind::ModeShift,
                stacks,
            } => *stacks += mode_shift_bonus,
            _ => {}
        }
    }

    push_move_history(entity, move_next as u8);
}
