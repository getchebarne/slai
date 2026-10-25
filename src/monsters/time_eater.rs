use crate::consts::MAX_MONSTER_MOVES;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_attack_debuff;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Ripple: block, then Vulnerable and Weak on the Character
static MOVE_RIPPLE: Move = make_move(
    "Ripple",
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 20 },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Vulnerable,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::BlockDebuff,
);

// A19+ Ripple also applies Frail
static MOVE_RIPPLE_A19: Move = make_move(
    "Ripple",
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 20 },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Vulnerable,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Frail,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::BlockDebuff,
);

// Haste: shed every debuff, then heal to half max HP
static MOVE_HASTE: Move = make_move(
    "Haste",
    &[
        Effect {
            kind: EffectKind::DebuffsClear,
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::HealthSet {
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 2,
                    rounding: Rounding::Truncate,
                    read_at: ReadAt::Now,
                },
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::Buff,
);

// A19+ Haste also gains 32 block
static MOVE_HASTE_A19: Move = make_move(
    "Haste",
    &[
        Effect {
            kind: EffectKind::DebuffsClear,
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::HealthSet {
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 2,
                    rounding: Rounding::Truncate,
                    read_at: ReadAt::Now,
                },
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::BlockGain { amount: 32 },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::Buff,
);

static MOVE_REVERBERATE_7: Move = move_attack("Reverberate", 7, 3);
static MOVE_REVERBERATE_8: Move = move_attack("Reverberate", 8, 3);
static MOVE_HEAD_SLAM_26: Move =
    move_attack_debuff("Head Slam", 26, ModifierKind::DrawReduction, 1);
static MOVE_HEAD_SLAM_32: Move =
    move_attack_debuff("Head Slam", 32, ModifierKind::DrawReduction, 1);

// A19+ Head Slam also adds two Slimed to the discard pile
static MOVE_HEAD_SLAM_32_A19: Move = make_move(
    "Head Slam",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 32,
                instances: 1,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::DrawReduction,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Slimed,
                card_pile: CardPile::Discard,
                count: 2,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    Intent::AttackDebuff {
        damage: 32,
        instances: 1,
    },
);

static MOVES_ASC0: [Move; 4] = [
    MOVE_REVERBERATE_7,
    MOVE_RIPPLE,
    MOVE_HEAD_SLAM_26,
    MOVE_HASTE,
];
static MOVES_ASC4: [Move; 4] = [
    MOVE_REVERBERATE_8,
    MOVE_RIPPLE,
    MOVE_HEAD_SLAM_32,
    MOVE_HASTE,
];
static MOVES_ASC19: [Move; 4] = [
    MOVE_REVERBERATE_8,
    MOVE_RIPPLE_A19,
    MOVE_HEAD_SLAM_32_A19,
    MOVE_HASTE_A19,
];

const IDX_MOVE_REVERBERATE: usize = 0;
const IDX_MOVE_RIPPLE: usize = 1;
const IDX_MOVE_HEAD_SLAM: usize = 2;
const IDX_MOVE_HASTE: usize = 3;

pub static TIME_EATER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::TimeEater,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (456, 456)), (9, (480, 480))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (4, &[&MOVES_ASC4]),
        (19, &[&MOVES_ASC19]),
    ],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::TimeWarp, 0)])],
};

pub fn get_next_move_time_eater(
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    health: u16,
    health_max: u16,
    rng: &mut impl Rng,
) -> usize {
    // The roll is drawn ahead of the Haste check, so a Haste turn draws it too
    let roll = rng.random_range(0..=99);

    // Below half HP Time Eater Hastes, once per combat
    if health < health_max / 2 && move_uses[IDX_MOVE_HASTE] == 0 {
        return IDX_MOVE_HASTE;
    }

    let last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);

    // Reverberate never runs three times in a row: its band re-rolls into the two above it
    let roll = if roll < 45 {
        if !move_history.ends_with(&[IDX_MOVE_REVERBERATE as u8, IDX_MOVE_REVERBERATE as u8]) {
            return IDX_MOVE_REVERBERATE;
        }
        rng.random_range(50..=99)
    } else {
        roll
    };

    // Head Slam never repeats: a repeat becomes Reverberate or Ripple
    if roll < 80 {
        return if last != Some(IDX_MOVE_HEAD_SLAM) {
            IDX_MOVE_HEAD_SLAM
        } else if rng.random_bool(0.66) {
            IDX_MOVE_REVERBERATE
        } else {
            IDX_MOVE_RIPPLE
        };
    }

    // Ripple never repeats: its band re-rolls into the two below it, where the last move blocks nothing
    if last != Some(IDX_MOVE_RIPPLE) {
        IDX_MOVE_RIPPLE
    } else if rng.random_range(0..=74) < 45 {
        IDX_MOVE_REVERBERATE
    } else {
        IDX_MOVE_HEAD_SLAM
    }
}
