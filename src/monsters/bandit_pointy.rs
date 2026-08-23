use crate::entity::Move;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVES_ASC0: [Move; 1] = [move_attack("Spit Web", 5, 2)];
static MOVES_ASC2: [Move; 1] = [move_attack("Spit Web", 6, 2)];

pub static BANDIT_POINTY: MonsterTemplate = MonsterTemplate {
    name: MonsterName::BanditPointy,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (30, 30)), (7, (34, 34))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2)],
    modifier_tiers: &[],
};
