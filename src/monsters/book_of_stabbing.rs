use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::entity::Entity;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::monsters::Move;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// One Multi-Stab template per hit count (hits = idx + 2, up to the effect cap);
// Single Stab closes the table
const NUM_MULTI_STAB_MOVES: usize = MAX_EFFECTS_PER_MOVE - 1;

const fn move_table(multi_damage: u16, single_damage: u16) -> [Move; NUM_MULTI_STAB_MOVES + 1] {
    let mut table = [make_move_attack("Single Stab", single_damage, 1); NUM_MULTI_STAB_MOVES + 1];
    let mut i = 0;
    while i < NUM_MULTI_STAB_MOVES {
        table[i] = make_move_attack("Multi-Stab", multi_damage, (i + 2) as u8);
        i += 1;
    }
    table
}

static MOVES_ASC0: [Move; NUM_MULTI_STAB_MOVES + 1] = move_table(6, 21);
static MOVES_ASC3: [Move; NUM_MULTI_STAB_MOVES + 1] = move_table(7, 24);

const IDX_MOVE_MULTI_STAB_LAST: usize = NUM_MULTI_STAB_MOVES - 1;
const IDX_MOVE_SINGLE_STAB: usize = NUM_MULTI_STAB_MOVES;

const fn is_multi_stab(idx: usize) -> bool {
    idx <= IDX_MOVE_MULTI_STAB_LAST
}

pub fn spawn_monster_book_of_stabbing(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (160, 164)
    } else {
        (168, 172)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 3 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC3
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::PainfulStabs, 1);

    make_entity_monster(
        MonsterName::BookOfStabbing,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}

pub fn get_next_move_book_of_stabbing(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    // Hit count starts at 2 and grows once per Multi-Stab pick; A18+ grows every turn
    let escalation = if ascension_level >= 18 {
        move_history.len()
    } else {
        move_history
            .iter()
            .filter(|&&m| is_multi_stab(m as usize))
            .count()
    };
    let idx_multi = escalation.min(IDX_MOVE_MULTI_STAB_LAST);

    let last = move_history.last().copied().map(|m| m as usize);
    if rng.random_range(0..=99) < 15 {
        if last == Some(IDX_MOVE_SINGLE_STAB) {
            idx_multi
        } else {
            IDX_MOVE_SINGLE_STAB
        }
    } else if move_history.len() >= 2
        && move_history[move_history.len() - 2..]
            .iter()
            .all(|&m| is_multi_stab(m as usize))
    {
        IDX_MOVE_SINGLE_STAB
    } else {
        idx_multi
    }
}
