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

// It Is Time's damage is the base; 5 more per earlier It Is Time, 30 more at most, locks in when the move is chosen
static MOVE_IT_IS_TIME_30: Move = move_attack("It Is Time", 30, 1);
static MOVE_IT_IS_TIME_40: Move = move_attack("It Is Time", 40, 1);

static MOVES_ASC0: [Move; 3] = [MOVE_GLARE, MOVE_IT_IS_TIME_30, MOVE_COUNT];
static MOVES_ASC3: [Move; 3] = [MOVE_GLARE, MOVE_IT_IS_TIME_40, MOVE_COUNT];

const IDX_MOVE_GLARE: usize = 0;
pub const IDX_MOVE_IT_IS_TIME: usize = 1;
const IDX_MOVE_COUNT: usize = 2;

pub static GIANT_HEAD: MonsterTemplate = MonsterTemplate {
    name: MonsterName::GiantHead,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (500, 500)), (8, (520, 520))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (3, &[&MOVES_ASC3])],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Slow, 0)])],
};

// Four Glares or Counts (three from A18), then It Is Time every turn; each Glare or Count is a 50/50 roll that never picks one a third time in a row
pub fn get_next_move_giant_head(
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let countdown = if ascension_level >= 18 { 3 } else { 4 };
    if move_uses[IDX_MOVE_GLARE] + move_uses[IDX_MOVE_COUNT] >= countdown {
        return IDX_MOVE_IT_IS_TIME;
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
