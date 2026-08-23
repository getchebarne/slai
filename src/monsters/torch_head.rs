use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVES: [Move; 1] = [move_attack("Tackle", 7, 1)];

pub static TORCH_HEAD: MonsterTemplate = MonsterTemplate {
    name: MonsterName::TorchHead,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (38, 40)), (9, (40, 45))],
    block_start: 0,
    move_tiers: &[(0, &MOVES)],
    modifier_tiers: &[(0, &[(ModifierKind::Minion, 1)])],
};

// Deterministic: always uses Tackle
