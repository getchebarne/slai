use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_ATTACK_9: Move = move_attack("Attack", 9, 1);
static MOVE_ATTACK_11: Move = move_attack("Attack", 11, 1);

// The third move does nothing; the blast behind it is Explosive's, not the move's
static MOVE_UNKNOWN: Move = make_move("Unknown", &[], Intent::Unknown);

static MOVES_ASC0: [Move; 2] = [MOVE_ATTACK_9, MOVE_UNKNOWN];
static MOVES_ASC2: [Move; 2] = [MOVE_ATTACK_11, MOVE_UNKNOWN];

const IDX_MOVE_ATTACK: usize = 0;
const IDX_MOVE_UNKNOWN: usize = 1;

pub static EXPLODER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Exploder,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (30, 30)), (7, (30, 35))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Explosive, 3)])],
};

// Attack on its first two turns, then the empty move
pub fn get_next_move_exploder(move_history: &[u8]) -> usize {
    if move_history.len() < 2 {
        IDX_MOVE_ATTACK
    } else {
        IDX_MOVE_UNKNOWN
    }
}
