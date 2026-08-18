use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::Move;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// One Multi-Stab template per hit count (hits = idx + 2, up to the effect cap);
// Single Stab closes the table
const NUM_MULTI_STAB_MOVES: usize = MAX_EFFECTS_PER_MOVE - 1;

const fn move_table(multi_damage: u16, single_damage: u16) -> [Move; NUM_MULTI_STAB_MOVES + 1] {
    let mut table = [move_attack("Single Stab", single_damage, 1); NUM_MULTI_STAB_MOVES + 1];
    let mut idx = 0;
    while idx < NUM_MULTI_STAB_MOVES {
        table[idx] = move_attack("Multi-Stab", multi_damage, (idx + 2) as u8);
        idx += 1;
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

pub static TEMPLATE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::BookOfStabbing,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (160, 164)), (8, (168, 172))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (3, &MOVES_ASC3)],
    modifier_tiers: &[(0, &[(ModifierKind::PainfulStabs, 1)])],
};

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

    let last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    if rng.random_range(0..=99) < 15 {
        if last == Some(IDX_MOVE_SINGLE_STAB) {
            idx_multi
        } else {
            IDX_MOVE_SINGLE_STAB
        }
    } else if move_history.len() >= 2
        && move_history[move_history.len() - 2..]
            .iter()
            .all(|&idx_move| is_multi_stab(idx_move as usize))
    {
        IDX_MOVE_SINGLE_STAB
    } else {
        idx_multi
    }
}
