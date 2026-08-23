use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::monsters::move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_MOCK: Move = make_move("Mock", &[], Intent::Unknown);
static MOVE_AGONIZING_10_W2: Move =
    move_attack_debuff("Agonizing Slash", 10, ModifierKind::Weak, 2);
static MOVE_AGONIZING_12_W2: Move =
    move_attack_debuff("Agonizing Slash", 12, ModifierKind::Weak, 2);
static MOVE_AGONIZING_12_W3: Move =
    move_attack_debuff("Agonizing Slash", 12, ModifierKind::Weak, 3);
static MOVE_CROSS_SLASH_15: Move = move_attack("Cross Slash", 15, 1);
static MOVE_CROSS_SLASH_17: Move = move_attack("Cross Slash", 17, 1);

static MOVES_ASC0: [Move; 3] = [MOVE_MOCK, MOVE_AGONIZING_10_W2, MOVE_CROSS_SLASH_15];
static MOVES_ASC2: [Move; 3] = [MOVE_MOCK, MOVE_AGONIZING_12_W2, MOVE_CROSS_SLASH_17];
static MOVES_ASC17: [Move; 3] = [MOVE_MOCK, MOVE_AGONIZING_12_W3, MOVE_CROSS_SLASH_17];

const IDX_MOVE_MOCK: usize = 0;
const IDX_MOVE_AGONIZING: usize = 1;
const IDX_MOVE_CROSS_SLASH: usize = 2;

pub static BANDIT_LEADER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::BanditLeader,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (35, 39)), (7, (37, 41))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

// Mock opener, then Agonizing/Cross alternating; A17+ chains Cross Slash twice
pub fn get_next_move_bandit_leader(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_MOCK;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    match last {
        IDX_MOVE_MOCK => IDX_MOVE_AGONIZING,
        IDX_MOVE_AGONIZING => IDX_MOVE_CROSS_SLASH,
        IDX_MOVE_CROSS_SLASH => {
            if ascension_level >= 17
                && !move_history
                    .ends_with(&[IDX_MOVE_CROSS_SLASH as u8, IDX_MOVE_CROSS_SLASH as u8])
            {
                IDX_MOVE_CROSS_SLASH
            } else {
                IDX_MOVE_AGONIZING
            }
        }
        _ => unreachable!("Bandit Leader unexpected move idx: {last}"),
    }
}
