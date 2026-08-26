use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::has_modifier;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

// Siphon Soul: equal Strength and Dexterity drain
const fn move_siphon_soul(stacks: i16) -> Move {
    make_move(
        "Siphon Soul",
        &[
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Dexterity,
                    stacks,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::DebuffPowerful,
    )
}

static MOVE_SLEEP: Move = make_move("Sleep", &[], Intent::Sleep);
static MOVE_WAKE_UP: Move = make_move(
    "Wake up",
    &[
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Asleep,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::Metallicize,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::Sleep,
);
static MOVE_STUNNED: Move = make_move("Stunned", &[], Intent::Stunned);
static MOVE_ATTACK_18: Move = move_attack("Attack", 18, 1);
static MOVE_ATTACK_20: Move = move_attack("Attack", 20, 1);
static MOVE_SIPHON_SOUL_1: Move = move_siphon_soul(-1);
static MOVE_SIPHON_SOUL_2: Move = move_siphon_soul(-2);

static MOVES_ASC0: [Move; 5] = [
    MOVE_SLEEP,
    MOVE_WAKE_UP,
    MOVE_STUNNED,
    MOVE_ATTACK_18,
    MOVE_SIPHON_SOUL_1,
];
static MOVES_ASC3: [Move; 5] = [
    MOVE_SLEEP,
    MOVE_WAKE_UP,
    MOVE_STUNNED,
    MOVE_ATTACK_20,
    MOVE_SIPHON_SOUL_1,
];
static MOVES_ASC18: [Move; 5] = [
    MOVE_SLEEP,
    MOVE_WAKE_UP,
    MOVE_STUNNED,
    MOVE_ATTACK_20,
    MOVE_SIPHON_SOUL_2,
];

const IDX_MOVE_SLEEP: usize = 0;
const IDX_MOVE_WAKE_UP: usize = 1;
pub const IDX_MOVE_STUNNED: usize = 2;
const IDX_MOVE_ATTACK: usize = 3;
pub const IDX_MOVE_SIPHON: usize = 4;

pub static LAGAVULIN: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Lagavulin,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (109, 111)), (8, (112, 115))],
    block_start: 8,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (3, &[&MOVES_ASC3]),
        (18, &[&MOVES_ASC18]),
    ],
    modifier_tiers: &[(
        0,
        &[
            modifier_fixed(ModifierKind::Asleep, 1),
            modifier_fixed(ModifierKind::Metallicize, 8),
        ],
    )],
};

pub fn get_next_move_lagavulin(
    move_current: Option<usize>,
    move_history: &[u8],
    modifiers: &Modifiers,
) -> usize {
    // Combat start: Sleep
    if move_current.is_none() {
        return IDX_MOVE_SLEEP;
    }

    if has_modifier(modifiers, ModifierKind::Asleep) {
        // Count trailing Sleep moves in history
        let trailing_sleeps = move_history
            .iter()
            .rev()
            .take_while(|&&m| m == IDX_MOVE_SLEEP as u8)
            .count();
        if trailing_sleeps < 2 {
            IDX_MOVE_SLEEP
        } else {
            IDX_MOVE_WAKE_UP
        }
    } else {
        // Awake rotation: 2 Attacks then Siphon Soul
        let trailing_attacks = move_history
            .iter()
            .rev()
            .take_while(|&&m| m == IDX_MOVE_ATTACK as u8)
            .count();
        if trailing_attacks >= 2 {
            IDX_MOVE_SIPHON
        } else {
            IDX_MOVE_ATTACK
        }
    }
}
