use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

const PLATED_ARMOR_START: i16 = 14;

const fn make_move_suck(damage: u16) -> Move {
    make_move(
        "Suck",
        &[Effect {
            kind: EffectKind::DamagePhysical {
                amount: damage,
                lifesteal: true,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        }],
        Intent::AttackBuff {
            damage,
            instances: 1,
        },
    )
}

static MOVE_FELL_18: Move = make_move_attack_debuff("Fell", 18, ModifierKind::Frail, 2);
static MOVE_FELL_21: Move = make_move_attack_debuff("Fell", 21, ModifierKind::Frail, 2);
static MOVE_DOUBLE_STRIKE_6: Move = make_move_attack("Double Strike", 6, 2);
static MOVE_DOUBLE_STRIKE_7: Move = make_move_attack("Double Strike", 7, 2);
static MOVE_SUCK_10: Move = make_move_suck(10);
static MOVE_SUCK_12: Move = make_move_suck(12);
static MOVE_STUNNED: Move = make_move("Stunned", &[], Intent::Stunned);

static MOVES_ASC0: [Move; 4] = [
    MOVE_FELL_18,
    MOVE_DOUBLE_STRIKE_6,
    MOVE_SUCK_10,
    MOVE_STUNNED,
];
static MOVES_ASC2: [Move; 4] = [
    MOVE_FELL_21,
    MOVE_DOUBLE_STRIKE_7,
    MOVE_SUCK_12,
    MOVE_STUNNED,
];

const IDX_MOVE_FELL: usize = 0;
const IDX_MOVE_DOUBLE_STRIKE: usize = 1;
const IDX_MOVE_SUCK: usize = 2;
pub const IDX_MOVE_STUNNED: usize = 3;

pub fn spawn_monster_shelled_parasite(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (68, 72)
    } else {
        (70, 75)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    let mut modifiers = MODIFIERS_ZERO;
    modifier_apply(
        &mut modifiers,
        ModifierKind::PlatedArmor,
        PLATED_ARMOR_START,
    );

    make_entity_monster(
        MonsterName::ShelledParasite,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: PLATED_ARMOR_START as u16,
        },
        modifiers,
        moves,
    )
}

pub fn get_next_move_shelled_parasite(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return if ascension_level >= 17 {
            IDX_MOVE_FELL
        } else if rng.random_bool(0.5) {
            IDX_MOVE_DOUBLE_STRIKE
        } else {
            IDX_MOVE_SUCK
        };
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    // The armor-break Stunned turn forces Fell next
    if last == IDX_MOVE_STUNNED {
        return IDX_MOVE_FELL;
    }

    // Fell never repeats: a low roll re-rolls into the upper range instead
    let mut roll = rng.random_range(0..=99);
    if roll < 20 && last == IDX_MOVE_FELL {
        roll = rng.random_range(20..=99);
    }

    if roll < 20 {
        IDX_MOVE_FELL
    } else if roll < 60 {
        if move_history.ends_with(&[IDX_MOVE_DOUBLE_STRIKE as u8, IDX_MOVE_DOUBLE_STRIKE as u8]) {
            IDX_MOVE_SUCK
        } else {
            IDX_MOVE_DOUBLE_STRIKE
        }
    } else if move_history.ends_with(&[IDX_MOVE_SUCK as u8, IDX_MOVE_SUCK as u8]) {
        IDX_MOVE_DOUBLE_STRIKE
    } else {
        IDX_MOVE_SUCK
    }
}
