use crate::consts::MAX_MONSTER_MOVES;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
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
use crate::monsters::snake_plant::MALLEABLE_BASE;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Attack Block: the hit, then block equal to its unscaled damage
const fn move_attack_block(damage: u16) -> Move {
    make_move(
        "Attack Block",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    instances: 1,
                    lifesteal: false,
                    if_poisoned: false,
                    scaled_at: ScaledAt::Now,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::BlockGain { amount: damage },
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::AttackBlock {
            damage,
            instances: 1,
        },
    )
}

// Attack Debuff: the hit, then Weak ahead of Vulnerable, so a lone Artifact stack negates the Weak
const fn move_attack_weak_vulnerable(damage: u16) -> Move {
    make_move(
        "Attack Debuff",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    instances: 1,
                    lifesteal: false,
                    if_poisoned: false,
                    scaled_at: ScaledAt::Now,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Weak,
                    stacks: 2,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierDelta {
                    kind: ModifierKind::Vulnerable,
                    stacks: 2,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::AttackDebuff {
            damage,
            instances: 1,
        },
    )
}

static MOVE_BIG_HIT_32: Move = move_attack("Big Hit", 32, 1);
static MOVE_BIG_HIT_38: Move = move_attack("Big Hit", 38, 1);
static MOVE_MULTI_HIT_7: Move = move_attack("Multi Hit", 7, 3);
static MOVE_MULTI_HIT_9: Move = move_attack("Multi Hit", 9, 3);
static MOVE_ATTACK_BLOCK_15: Move = move_attack_block(15);
static MOVE_ATTACK_BLOCK_16: Move = move_attack_block(16);
static MOVE_ATTACK_DEBUFF_10: Move = move_attack_weak_vulnerable(10);
static MOVE_ATTACK_DEBUFF_12: Move = move_attack_weak_vulnerable(12);

// Implant: one Parasite into the deck
static MOVE_IMPLANT: Move = make_move(
    "Implant",
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Parasite,
            card_pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    Intent::DebuffPowerful,
);

static MOVES_ASC0: [Move; 5] = [
    MOVE_BIG_HIT_32,
    MOVE_MULTI_HIT_7,
    MOVE_ATTACK_BLOCK_15,
    MOVE_ATTACK_DEBUFF_10,
    MOVE_IMPLANT,
];
static MOVES_ASC2: [Move; 5] = [
    MOVE_BIG_HIT_38,
    MOVE_MULTI_HIT_9,
    MOVE_ATTACK_BLOCK_16,
    MOVE_ATTACK_DEBUFF_12,
    MOVE_IMPLANT,
];

const IDX_MOVE_BIG_HIT: usize = 0;
const IDX_MOVE_MULTI_HIT: usize = 1;
const IDX_MOVE_ATTACK_BLOCK: usize = 2;
const IDX_MOVE_ATTACK_DEBUFF: usize = 3;
pub const IDX_MOVE_IMPLANT: usize = 4;

pub static WRITHING_MASS: MonsterTemplate = MonsterTemplate {
    name: MonsterName::WrithingMass,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (160, 160)), (7, (175, 175))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[(
        0,
        &[
            modifier_fixed(ModifierKind::Reactive, 1),
            modifier_fixed(ModifierKind::Malleable, MALLEABLE_BASE),
        ],
    )],
};

pub fn get_next_move_writhing_mass(
    move_current: Option<usize>,
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    rng: &mut impl Rng,
) -> usize {
    // The opening move is never Big Hit or Implant
    if move_current.is_none() {
        let roll = rng.random_range(0..=99);
        return if roll < 33 {
            IDX_MOVE_MULTI_HIT
        } else if roll < 66 {
            IDX_MOVE_ATTACK_BLOCK
        } else {
            IDX_MOVE_ATTACK_DEBUFF
        };
    }
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);

    // A blocked band draws again from the range it names, or a coin decides; every draw checks the same last move
    let mut roll = rng.random_range(0..=99);
    loop {
        roll = if roll < 10 {
            if move_last != Some(IDX_MOVE_BIG_HIT) {
                return IDX_MOVE_BIG_HIT;
            }
            rng.random_range(10..=99)
        } else if roll < 20 {
            // Implant comes once per combat, spent when it executes, and never twice in a row
            if move_uses[IDX_MOVE_IMPLANT] == 0 && move_last != Some(IDX_MOVE_IMPLANT) {
                return IDX_MOVE_IMPLANT;
            }

            // This coin's Big Hit may follow a Big Hit
            if rng.random_bool(0.1) {
                return IDX_MOVE_BIG_HIT;
            }
            rng.random_range(20..=99)
        } else if roll < 40 {
            if move_last != Some(IDX_MOVE_ATTACK_DEBUFF) {
                return IDX_MOVE_ATTACK_DEBUFF;
            }
            if rng.random_bool(0.4) {
                rng.random_range(0..=19)
            } else {
                rng.random_range(40..=99)
            }
        } else if roll < 70 {
            if move_last != Some(IDX_MOVE_MULTI_HIT) {
                return IDX_MOVE_MULTI_HIT;
            }
            if rng.random_bool(0.3) {
                return IDX_MOVE_ATTACK_BLOCK;
            }
            rng.random_range(0..=39)
        } else {
            if move_last != Some(IDX_MOVE_ATTACK_BLOCK) {
                return IDX_MOVE_ATTACK_BLOCK;
            }
            rng.random_range(0..=69)
        };
    }
}
