use crate::entity::Move;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_TACKLE_5: Move = move_attack("Tackle", 5, 1);
static MOVE_TACKLE_6: Move = move_attack("Tackle", 6, 1);
static MOVES_ASC0: [Move; 1] = [MOVE_TACKLE_5];
static MOVES_ASC2: [Move; 1] = [MOVE_TACKLE_6];
static MOVES_ASC17: [Move; 1] = [MOVE_TACKLE_6];

pub static SLIME_SPIKE_SMALL: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlimeSpikeSmall,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (10, 14)), (7, (11, 15))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[],
};
