use crate::consts::MAX_MONSTER_MOVES;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_GLARE: Move = move_debuff("Glare", ModifierKind::Weak, 1, Intent::Debuff);
static MOVE_COUNT: Move = move_attack("Count", 13, 1);

// It Is Time hits 5 more each time, up to 30 more; one entry per step
static MOVES_ASC0: [Move; 9] = [
    MOVE_GLARE,
    MOVE_COUNT,
    move_attack("It Is Time", 30, 1),
    move_attack("It Is Time", 35, 1),
    move_attack("It Is Time", 40, 1),
    move_attack("It Is Time", 45, 1),
    move_attack("It Is Time", 50, 1),
    move_attack("It Is Time", 55, 1),
    move_attack("It Is Time", 60, 1),
];
static MOVES_ASC3: [Move; 9] = [
    MOVE_GLARE,
    MOVE_COUNT,
    move_attack("It Is Time", 40, 1),
    move_attack("It Is Time", 45, 1),
    move_attack("It Is Time", 50, 1),
    move_attack("It Is Time", 55, 1),
    move_attack("It Is Time", 60, 1),
    move_attack("It Is Time", 65, 1),
    move_attack("It Is Time", 70, 1),
];

const IDX_MOVE_GLARE: usize = 0;
const IDX_MOVE_COUNT: usize = 1;
const IDX_MOVE_IT_IS_TIME_FIRST: usize = 2;
const IDX_MOVE_IT_IS_TIME_LAST: usize = 8;

pub static GIANT_HEAD: MonsterTemplate = MonsterTemplate {
    name: MonsterName::GiantHead,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (500, 500)), (8, (520, 520))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (3, &[&MOVES_ASC3])],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Slow, 0)])],
};

// Four Glares or Counts (three from A18), then It Is Time every turn, each the table's next entry until the last; each Glare or Count is a 50/50 roll that never picks one a third time in a row
pub fn get_next_move_giant_head(
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let countdown = if ascension_level >= 18 { 3 } else { 4 };
    if move_uses[IDX_MOVE_GLARE] + move_uses[IDX_MOVE_COUNT] >= countdown {
        let it_is_time_uses: usize = move_uses
            [IDX_MOVE_IT_IS_TIME_FIRST..=IDX_MOVE_IT_IS_TIME_LAST]
            .iter()
            .map(|&uses| usize::from(uses))
            .sum();
        return (IDX_MOVE_IT_IS_TIME_FIRST + it_is_time_uses).min(IDX_MOVE_IT_IS_TIME_LAST);
    }

    let glare_twice = move_history.ends_with(&[IDX_MOVE_GLARE as u8, IDX_MOVE_GLARE as u8]);
    let count_twice = move_history.ends_with(&[IDX_MOVE_COUNT as u8, IDX_MOVE_COUNT as u8]);
    if rng.random_range(0..=99) < 50 {
        if glare_twice {
            IDX_MOVE_COUNT
        } else {
            IDX_MOVE_GLARE
        }
    } else if count_twice {
        IDX_MOVE_GLARE
    } else {
        IDX_MOVE_COUNT
    }
}
