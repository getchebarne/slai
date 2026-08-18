use crate::entity::Move;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_PUNCTURE_9: Move = move_attack("Puncture", 9, 1);
static MOVE_PUNCTURE_10: Move = move_attack("Puncture", 10, 1);
static MOVES_ASC0: [Move; 1] = [MOVE_PUNCTURE_9];
static MOVES_ASC2: [Move; 1] = [MOVE_PUNCTURE_10];
static MOVES_ASC17: [Move; 1] = [MOVE_PUNCTURE_10];

pub static TEMPLATE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::GremlinThief,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (10, 14)), (7, (11, 15))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};
