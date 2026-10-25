use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

// The damage is the first attack's; 10 per earlier attack is added when the move is chosen
static MOVES_ASC0: [Move; 1] = [move_attack("Attack", 30, 1)];
static MOVES_ASC2: [Move; 1] = [move_attack("Attack", 40, 1)];

pub const IDX_MOVE_ATTACK: usize = 0;

pub static TRANSIENT: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Transient,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (999, 999))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
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
