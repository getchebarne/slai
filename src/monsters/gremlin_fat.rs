use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_SMASH_4_W1: Move = move_attack_debuff("Smash", 4, ModifierKind::Weak, 1);
static MOVE_SMASH_5_W1: Move = move_attack_debuff("Smash", 5, ModifierKind::Weak, 1);
static MOVE_SMASH_5_W1_F1: Move = make_move(
    "Smash",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 5,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Frail,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::AttackDebuff {
        damage: 5,
        instances: 1,
    },
);
static MOVES_ASC0: [Move; 1] = [MOVE_SMASH_4_W1];
static MOVES_ASC2: [Move; 1] = [MOVE_SMASH_5_W1];
static MOVES_ASC17: [Move; 1] = [MOVE_SMASH_5_W1_F1];

pub static GREMLIN_FAT: MonsterTemplate = MonsterTemplate {
    name: MonsterName::GremlinFat,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (13, 17)), (7, (14, 18))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[],
};
