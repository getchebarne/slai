use crate::consts::MAX_MONSTER_MOVES;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_ATTACK_7: Move = move_attack("Attack", 7, 1);
static MOVE_ATTACK_9: Move = move_attack("Attack", 9, 1);
static MOVE_BUFF: Move = move_buff("Buff", ModifierKind::Thorns, 2);

static MOVES_ASC0: [Move; 2] = [MOVE_ATTACK_7, MOVE_BUFF];
static MOVES_ASC2: [Move; 2] = [MOVE_ATTACK_9, MOVE_BUFF];

const IDX_MOVE_ATTACK: usize = 0;
const IDX_MOVE_BUFF: usize = 1;

pub static SPIKER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Spiker,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (42, 56)), (7, (44, 60))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[
        (0, &[modifier_fixed(ModifierKind::Thorns, 3)]),
        (2, &[modifier_fixed(ModifierKind::Thorns, 4)]),
        (17, &[modifier_fixed(ModifierKind::Thorns, 7)]),
    ],
};

// From the sixth Buff on it attacks every turn; before that one roll, Attack below 50 unless it was the last move
pub fn get_next_move_spiker(
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    rng: &mut impl Rng,
) -> usize {
    if move_uses[IDX_MOVE_BUFF] >= 6 {
        return IDX_MOVE_ATTACK;
    }
    let roll = rng.random_range(0..=99);
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    if roll < 50 && move_last != Some(IDX_MOVE_ATTACK) {
        IDX_MOVE_ATTACK
    } else {
        IDX_MOVE_BUFF
    }
}
