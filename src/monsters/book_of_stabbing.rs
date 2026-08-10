use crate::entity::Entity;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Multi-Stab is declared at one hit; move_update rewrites the hit count each pick
static MOVE_MULTI_STAB_6: Move = make_move_attack("Multi-Stab", 6, 1);
static MOVE_MULTI_STAB_7: Move = make_move_attack("Multi-Stab", 7, 1);
static MOVE_SINGLE_STAB_21: Move = make_move_attack("Single Stab", 21, 1);
static MOVE_SINGLE_STAB_24: Move = make_move_attack("Single Stab", 24, 1);

static MOVES_ASC0: [Move; 2] = [MOVE_MULTI_STAB_6, MOVE_SINGLE_STAB_21];
static MOVES_ASC3: [Move; 2] = [MOVE_MULTI_STAB_7, MOVE_SINGLE_STAB_24];

pub const IDX_MOVE_MULTI_STAB: usize = 0;
const IDX_MOVE_SINGLE_STAB: usize = 1;

// Stab count starts at 2 and grows once per Multi-Stab pick; A18+ grows every turn.
// The move's own effects_len (seeded at 1) carries the running count
pub fn multi_stab_hits(effects_len_prev: usize, turns_taken: usize, ascension_level: u8) -> usize {
    if ascension_level >= 18 {
        2 + turns_taken
    } else {
        effects_len_prev + 1
    }
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

pub fn get_next_move_book_of_stabbing(move_history: &[u8], rng: &mut impl Rng) -> usize {
    let last = move_history.last().copied().map(|m| m as usize);
    if rng.random_range(0..=99) < 15 {
        if last == Some(IDX_MOVE_SINGLE_STAB) {
            IDX_MOVE_MULTI_STAB
        } else {
            IDX_MOVE_SINGLE_STAB
        }
    } else if move_history.ends_with(&[IDX_MOVE_MULTI_STAB as u8, IDX_MOVE_MULTI_STAB as u8]) {
        IDX_MOVE_SINGLE_STAB
    } else {
        IDX_MOVE_MULTI_STAB
    }
}
