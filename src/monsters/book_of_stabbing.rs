use crate::consts::MAX_MONSTER_MOVES;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::Move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Multi-Stab's hit count is a placeholder, locked in when the move is chosen
static MOVES_ASC0: [Move; 2] = [
    move_attack("Multi-Stab", 6, 0),
    move_attack("Single Stab", 21, 1),
];
static MOVES_ASC3: [Move; 2] = [
    move_attack("Multi-Stab", 7, 0),
    move_attack("Single Stab", 24, 1),
];

pub const IDX_MOVE_MULTI_STAB: usize = 0;
const IDX_MOVE_SINGLE_STAB: usize = 1;

pub static BOOK_OF_STABBING: MonsterTemplate = MonsterTemplate {
    name: MonsterName::BookOfStabbing,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (160, 164)), (8, (168, 172))],
    block_start: 0,
    move_tiers: &[(0, &[&MOVES_ASC0]), (3, &[&MOVES_ASC3])],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::PainfulStabs, 1)])],
};

pub fn get_next_move_book_of_stabbing(move_history: &[u8], rng: &mut impl Rng) -> usize {
    let last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    if rng.random_range(0..=99) < 15 {
        if last == Some(IDX_MOVE_SINGLE_STAB) {
            IDX_MOVE_MULTI_STAB
        } else {
            IDX_MOVE_SINGLE_STAB
        }
    } else if move_history.len() >= 2
        && move_history[move_history.len() - 2..]
            .iter()
            .all(|&idx_move| idx_move as usize == IDX_MOVE_MULTI_STAB)
    {
        IDX_MOVE_SINGLE_STAB
    } else {
        IDX_MOVE_MULTI_STAB
    }
}

// Multi-Stab starts at 2 hits and grows once per earlier Multi-Stab; A18+ grows once per earlier move of either kind
pub fn multi_stab_instances(move_uses: &[u8; MAX_MONSTER_MOVES], ascension_level: u8) -> u8 {
    let instances = 2u8.saturating_add(move_uses[IDX_MOVE_MULTI_STAB]);
    if ascension_level >= 18 {
        instances.saturating_add(move_uses[IDX_MOVE_SINGLE_STAB])
    } else {
        instances
    }
}
