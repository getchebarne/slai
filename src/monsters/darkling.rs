use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_block;
use crate::types::DeltaSign;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

const MOVE_HARDEN: Move = move_block("Harden", 12);

// Harden from A17: the block, then 2 Strength
const MOVE_HARDEN_STRENGTH: Move = make_move(
    "Harden",
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 12 },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Strength,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::BlockBuff,
);

// Count: the half-dead Darkling's turn before its rebirth, which does nothing
const MOVE_COUNT: Move = make_move("Count", &[], Intent::Unknown);

// Reincarnate: the Darkling leaves half-dead, heals half its max HP and regrows; Philosopher's Stone adds 1 Strength
const MOVE_REINCARNATE: Move = make_move(
    "Reincarnate",
    &[
        Effect {
            kind: EffectKind::HalfDeadRebirth,
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
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
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::Regrow,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::PhilosopherStoneProc,
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::Buff,
);

// One table per spawn-rolled Nip damage, each holding the five moves at the same indices
const fn moves(chomp: u16, harden: Move, nip: u16) -> [Move; 5] {
    [
        move_attack("Chomp", chomp, 2),
        harden,
        move_attack("Nip", nip, 1),
        MOVE_COUNT,
        MOVE_REINCARNATE,
    ]
}

static MOVES_ASC0: [&[Move]; 5] = [
    &moves(8, MOVE_HARDEN, 7),
    &moves(8, MOVE_HARDEN, 8),
    &moves(8, MOVE_HARDEN, 9),
    &moves(8, MOVE_HARDEN, 10),
    &moves(8, MOVE_HARDEN, 11),
];
static MOVES_ASC2: [&[Move]; 5] = [
    &moves(9, MOVE_HARDEN, 9),
    &moves(9, MOVE_HARDEN, 10),
    &moves(9, MOVE_HARDEN, 11),
    &moves(9, MOVE_HARDEN, 12),
    &moves(9, MOVE_HARDEN, 13),
];
static MOVES_ASC17: [&[Move]; 5] = [
    &moves(9, MOVE_HARDEN_STRENGTH, 9),
    &moves(9, MOVE_HARDEN_STRENGTH, 10),
    &moves(9, MOVE_HARDEN_STRENGTH, 11),
    &moves(9, MOVE_HARDEN_STRENGTH, 12),
    &moves(9, MOVE_HARDEN_STRENGTH, 13),
];

const IDX_MOVE_CHOMP: usize = 0;
const IDX_MOVE_HARDEN: usize = 1;
const IDX_MOVE_NIP: usize = 2;
pub const IDX_MOVE_COUNT: usize = 3;
const IDX_MOVE_REINCARNATE: usize = 4;

pub static DARKLING: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Darkling,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (48, 56)), (7, (50, 59))],
    block_tiers: &[],
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Regrow, 1)])],
};

// Every roll draws, a half-dead Darkling's too, which always picks Reincarnate; Chomp comes only from an even slot
pub fn get_next_move_darkling(
    move_current: Option<usize>,
    move_history: &[u8],
    slot: usize,
    half_dead: bool,
    rng: &mut impl Rng,
) -> usize {
    let mut roll = rng.random_range(0..=99);
    if half_dead {
        return IDX_MOVE_REINCARNATE;
    }

    // The opening move is Harden or Nip, never Chomp
    if move_current.is_none() {
        return if roll < 50 {
            IDX_MOVE_HARDEN
        } else {
            IDX_MOVE_NIP
        };
    }
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);

    // Chomp never twice in a row (else a draw from 40 up), Harden never twice (else Nip), Nip never three times (else a fresh draw)
    loop {
        roll = if roll < 40 {
            if move_last != Some(IDX_MOVE_CHOMP) && slot.is_multiple_of(2) {
                return IDX_MOVE_CHOMP;
            }
            rng.random_range(40..=99)
        } else if roll < 70 {
            return if move_last != Some(IDX_MOVE_HARDEN) {
                IDX_MOVE_HARDEN
            } else {
                IDX_MOVE_NIP
            };
        } else if !move_history.ends_with(&[IDX_MOVE_NIP as u8, IDX_MOVE_NIP as u8]) {
            return IDX_MOVE_NIP;
        } else {
            rng.random_range(0..=99)
        };
    }
}
