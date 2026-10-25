use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

// 10 more per attack; Fading ends it after its fifth (sixth from A17), just after it picks the last entry
static MOVES_ASC0: [Move; 6] = [
    move_attack("Attack", 30, 1),
    move_attack("Attack", 40, 1),
    move_attack("Attack", 50, 1),
    move_attack("Attack", 60, 1),
    move_attack("Attack", 70, 1),
    move_attack("Attack", 80, 1),
];
static MOVES_ASC2: [Move; 6] = [
    move_attack("Attack", 40, 1),
    move_attack("Attack", 50, 1),
    move_attack("Attack", 60, 1),
    move_attack("Attack", 70, 1),
    move_attack("Attack", 80, 1),
    move_attack("Attack", 90, 1),
];
static MOVES_ASC17: [Move; 7] = [
    move_attack("Attack", 40, 1),
    move_attack("Attack", 50, 1),
    move_attack("Attack", 60, 1),
    move_attack("Attack", 70, 1),
    move_attack("Attack", 80, 1),
    move_attack("Attack", 90, 1),
    move_attack("Attack", 100, 1),
];

pub static TRANSIENT: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Transient,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (999, 999))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[
        (
            0,
            &[
                modifier_fixed(ModifierKind::Fading, 5),
                modifier_fixed(ModifierKind::Shifting, 1),
            ],
        ),
        (
            17,
            &[
                modifier_fixed(ModifierKind::Fading, 6),
                modifier_fixed(ModifierKind::Shifting, 1),
            ],
        ),
    ],
};

// Each turn takes the table's next attack
pub fn get_next_move_transient(move_history: &[u8]) -> usize {
    move_history.len()
}
