use crate::consts::MAX_MONSTER_MOVES;
use crate::effect::Effect;
use crate::effect::EffectKind;
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
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_BLOOD_SHOTS_12: Move = move_attack("Blood Shots", 2, 12);
static MOVE_BLOOD_SHOTS_15: Move = move_attack("Blood Shots", 2, 15);
static MOVE_ECHO_40: Move = move_attack("Echo", 40, 1);
static MOVE_ECHO_45: Move = move_attack("Echo", 45, 1);

// One of Debilitate's debuffs: 2 stacks on the Character
const fn debilitate_debuff(mod_kind: ModifierKind) -> Effect {
    Effect {
        kind: EffectKind::ModifierDelta {
            kind: mod_kind,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_CHARACTER,
    }
}

// One of Debilitate's status Cards: unupgraded, at its own random spot in the draw pile, never on top
const fn debilitate_card(card_name: CardName) -> Effect {
    Effect {
        kind: EffectKind::CardAdd {
            card_name,
            card_pile: CardPile::Draw,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }
}

// Debilitate: Vulnerable, Weak and Frail on the Character, then a Dazed, a Slimed, a Wound, a Burn and a Void into the draw pile
static MOVE_DEBILITATE: Move = make_move(
    "Debilitate",
    &[
        debilitate_debuff(ModifierKind::Vulnerable),
        debilitate_debuff(ModifierKind::Weak),
        debilitate_debuff(ModifierKind::Frail),
        debilitate_card(CardName::Dazed),
        debilitate_card(CardName::Slimed),
        debilitate_card(CardName::Wound),
        debilitate_card(CardName::Burn),
        debilitate_card(CardName::Void),
    ],
    Intent::DebuffPowerful,
);

// Buff: a negative Strength comes off, then 2 Strength, then the tier's extra
const fn move_buff_tier(mod_kind: ModifierKind, stacks: i16) -> Move {
    make_move(
        "Buff",
        &[
            Effect {
                kind: EffectKind::ModifierStrengthNegativeRemove,
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
            Effect {
                kind: EffectKind::ModifierDelta {
                    kind: mod_kind,
                    stacks,
                },
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::Buff,
    )
}

static MOVE_BUFF_ARTIFACT: Move = move_buff_tier(ModifierKind::Artifact, 2);
static MOVE_BUFF_BEAT_OF_DEATH: Move = move_buff_tier(ModifierKind::BeatOfDeath, 1);
static MOVE_BUFF_PAINFUL_STABS: Move = move_buff_tier(ModifierKind::PainfulStabs, 1);
static MOVE_BUFF_STRENGTH_10: Move = move_buff_tier(ModifierKind::Strength, 10);
static MOVE_BUFF_STRENGTH_50: Move = move_buff_tier(ModifierKind::Strength, 50);

// One Buff entry per tier, in the order they come; the last repeats
static MOVES_ASC0: [Move; 8] = [
    MOVE_BLOOD_SHOTS_12,
    MOVE_ECHO_40,
    MOVE_DEBILITATE,
    MOVE_BUFF_ARTIFACT,
    MOVE_BUFF_BEAT_OF_DEATH,
    MOVE_BUFF_PAINFUL_STABS,
    MOVE_BUFF_STRENGTH_10,
    MOVE_BUFF_STRENGTH_50,
];
static MOVES_ASC4: [Move; 8] = [
    MOVE_BLOOD_SHOTS_15,
    MOVE_ECHO_45,
    MOVE_DEBILITATE,
    MOVE_BUFF_ARTIFACT,
    MOVE_BUFF_BEAT_OF_DEATH,
    MOVE_BUFF_PAINFUL_STABS,
    MOVE_BUFF_STRENGTH_10,
    MOVE_BUFF_STRENGTH_50,
];

const IDX_MOVE_BLOOD_SHOTS: usize = 0;
const IDX_MOVE_ECHO: usize = 1;
const IDX_MOVE_DEBILITATE: usize = 2;
const IDX_MOVE_BUFF_FIRST: usize = 3;
const IDX_MOVE_BUFF_LAST: usize = 7;

const STACKS_INVINCIBLE_BASE: i16 = 300;
const STACKS_INVINCIBLE_A19: i16 = 200;

// Turn start re-reads this to reset Invincible
pub fn stacks_invincible(ascension_level: u8) -> i16 {
    if ascension_level < 19 {
        STACKS_INVINCIBLE_BASE
    } else {
        STACKS_INVINCIBLE_A19
    }
}

pub static CORRUPT_HEART: MonsterTemplate = MonsterTemplate {
    name: MonsterName::CorruptHeart,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (750, 750)), (9, (800, 800))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (4, &[&MOVES_ASC4])],
    modifier_tiers: &[
        (
            0,
            &[
                modifier_fixed(ModifierKind::Invincible, STACKS_INVINCIBLE_BASE),
                modifier_fixed(ModifierKind::BeatOfDeath, 1),
            ],
        ),
        (
            19,
            &[
                modifier_fixed(ModifierKind::Invincible, STACKS_INVINCIBLE_A19),
                modifier_fixed(ModifierKind::BeatOfDeath, 2),
            ],
        ),
    ],
};

// Debilitate opens; then, forever, one attack by a coin flip, the other attack, and a Buff whose tier counts the Buffs before it
pub fn get_next_move_corrupt_heart(
    move_current: Option<usize>,
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_DEBILITATE;
    }

    // The attacks since the last Debilitate or Buff place the pick in its cycle
    let attacks_in_cycle = move_history
        .iter()
        .rev()
        .take_while(|&&idx_move| matches!(idx_move as usize, IDX_MOVE_BLOOD_SHOTS | IDX_MOVE_ECHO))
        .count();
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    match attacks_in_cycle {
        0 => {
            if rng.random_bool(0.5) {
                IDX_MOVE_BLOOD_SHOTS
            } else {
                IDX_MOVE_ECHO
            }
        }
        1 => {
            if move_last == Some(IDX_MOVE_ECHO) {
                IDX_MOVE_BLOOD_SHOTS
            } else {
                IDX_MOVE_ECHO
            }
        }
        _ => {
            let buff_uses: usize = move_uses[IDX_MOVE_BUFF_FIRST..=IDX_MOVE_BUFF_LAST]
                .iter()
                .map(|&uses| usize::from(uses))
                .sum();
            (IDX_MOVE_BUFF_FIRST + buff_uses).min(IDX_MOVE_BUFF_LAST)
        }
    }
}
