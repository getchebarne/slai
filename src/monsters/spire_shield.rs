use crate::consts::MAX_MONSTER_MOVES;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Smash: one hit, then the Shield gains block equal to the hit as computed
const fn move_smash(damage: u16) -> Move {
    make_move(
        "Smash",
        &[Effect {
            kind: EffectKind::DamagePhysical {
                amount: damage,
                instances: 1,
                lifesteal: false,
                if_poisoned: false,
                block_from_hit: true,
                scaled_at: ScaledAt::Now,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        }],
        Intent::AttackBlock {
            damage,
            instances: 1,
        },
    )
}

// The A18 Smash: one hit, then a flat 99 block on the Shield
static MOVE_SMASH_38_BLOCK_99: Move = make_move(
    "Smash",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 38,
                instances: 1,
                lifesteal: false,
                if_poisoned: false,
                block_from_hit: false,
                scaled_at: ScaledAt::Now,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::BlockGain { amount: 99 },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::AttackBlock {
        damage: 38,
        instances: 1,
    },
);

// Fortify: 30 block to every living Monster, the Shield included
static MOVE_FORTIFY: Move = make_move(
    "Fortify",
    &[Effect {
        kind: EffectKind::BlockGain { amount: 30 },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    Intent::Block,
);

static MOVE_BASH_12: Move = move_attack_debuff("Bash", 12, ModifierKind::Strength, -1);
static MOVE_BASH_14: Move = move_attack_debuff("Bash", 14, ModifierKind::Strength, -1);
static MOVE_SMASH_34: Move = move_smash(34);
static MOVE_SMASH_38: Move = move_smash(38);

static MOVES_ASC0: [Move; 3] = [MOVE_BASH_12, MOVE_FORTIFY, MOVE_SMASH_34];
static MOVES_ASC3: [Move; 3] = [MOVE_BASH_14, MOVE_FORTIFY, MOVE_SMASH_38];
static MOVES_ASC18: [Move; 3] = [MOVE_BASH_14, MOVE_FORTIFY, MOVE_SMASH_38_BLOCK_99];

const IDX_MOVE_BASH: usize = 0;
const IDX_MOVE_FORTIFY: usize = 1;
const IDX_MOVE_SMASH: usize = 2;

pub static SPIRE_SHIELD: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SpireShield,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (110, 110)), (8, (125, 125))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (3, &[&MOVES_ASC3]),
        (18, &[&MOVES_ASC18]),
    ],
    // The fight opens with the Character facing the Spear, so the Shield stands behind it
    modifier_tiers: &[
        (
            0,
            &[
                modifier_fixed(ModifierKind::Artifact, 1),
                modifier_fixed(ModifierKind::BackAttack, 1),
            ],
        ),
        (
            18,
            &[
                modifier_fixed(ModifierKind::Artifact, 2),
                modifier_fixed(ModifierKind::BackAttack, 1),
            ],
        ),
    ],
};

// A three-move cycle: a coin picks Fortify or Bash, the next move is the other one, then Smash
pub fn get_next_move_spire_shield(
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    rng: &mut impl Rng,
) -> usize {
    // The cycle's position counts executed moves, which outlast the history's 64-entry window
    let moves_used: usize = move_uses.iter().map(|&uses| usize::from(uses)).sum();
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    match moves_used % 3 {
        0 => {
            if rng.random_bool(0.5) {
                IDX_MOVE_FORTIFY
            } else {
                IDX_MOVE_BASH
            }
        }
        1 => {
            if move_last == Some(IDX_MOVE_BASH) {
                IDX_MOVE_FORTIFY
            } else {
                IDX_MOVE_BASH
            }
        }
        _ => IDX_MOVE_SMASH,
    }
}
