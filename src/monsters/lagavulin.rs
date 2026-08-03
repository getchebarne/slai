use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::has_modifier;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Siphon Soul: equal Strength and Dexterity drain
const fn make_move_siphon_soul(stacks: i16) -> Move {
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
static MOVE_ATTACK_18: Move = make_move_attack("Attack", 18, 1);
static MOVE_ATTACK_20: Move = make_move_attack("Attack", 20, 1);
static MOVE_SIPHON_SOUL_1: Move = make_move_siphon_soul(-1);
static MOVE_SIPHON_SOUL_2: Move = make_move_siphon_soul(-2);

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

pub fn spawn_monster_lagavulin(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (109, 111)
    } else {
        (112, 115)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 3 {
        &MOVES_ASC0
    } else if ascension_level < 18 {
        &MOVES_ASC3
    } else {
        &MOVES_ASC18
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Asleep, 1);
    modifier_apply(&mut modifiers, ModifierKind::Metallicize, 8);

    make_entity_monster(
        MonsterName::Lagavulin,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 8,
        },
        modifiers,
        moves,
    )
}

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
