use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::monsters::move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_BITE: Move = move_attack("Bite", 6, 1);
static MOVE_GROW_3: Move = move_buff("Grow", ModifierKind::Strength, 3);
static MOVE_GROW_4: Move = move_buff("Grow", ModifierKind::Strength, 4);
static MOVE_GROW_5: Move = move_buff("Grow", ModifierKind::Strength, 5);
static MOVES_ASC0: [Move; 2] = [MOVE_GROW_3, MOVE_BITE];
static MOVES_ASC2: [Move; 2] = [MOVE_GROW_4, MOVE_BITE];
static MOVES_ASC17: [Move; 2] = [MOVE_GROW_5, MOVE_BITE];

const IDX_MOVE_GROW: usize = 0;
const IDX_MOVE_BITE: usize = 1;

pub static FUNGI_BEAST: MonsterTemplate = MonsterTemplate {
    name: MonsterName::FungiBeast,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (22, 28)), (7, (24, 28))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[(0, &[(ModifierKind::SporeCloud, 2)])],
};

pub fn get_next_move_fungi_beast(move_history: &[u8], rng: &mut impl Rng) -> usize {
    let roll = rng.random_range(0..=99);
    if roll < 60 {
        if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_GROW
        } else {
            IDX_MOVE_BITE
        }
    } else if move_history.last().copied() == Some(IDX_MOVE_GROW as u8) {
        IDX_MOVE_BITE
    } else {
        IDX_MOVE_GROW
    }
}
