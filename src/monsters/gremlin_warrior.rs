use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_SCRATCH_4: Move = move_attack("Scratch", 4, 1);
static MOVE_SCRATCH_5: Move = move_attack("Scratch", 5, 1);
static MOVES_ASC0: [Move; 1] = [MOVE_SCRATCH_4];
static MOVES_ASC2: [Move; 1] = [MOVE_SCRATCH_5];
static MOVES_ASC17: [Move; 1] = [MOVE_SCRATCH_5];

pub static GREMLIN_WARRIOR: MonsterTemplate = MonsterTemplate {
    name: MonsterName::GremlinWarrior,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (20, 24)), (7, (21, 25))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[
        (0, &[(ModifierKind::Angry, 1)]),
        (17, &[(ModifierKind::Angry, 2)]),
    ],
};
