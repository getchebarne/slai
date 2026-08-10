use crate::effect::Effect;
use crate::effect::EffectKind;
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
use crate::monsters::make_move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

const fn make_move_go_airborne(flight: i16) -> Move {
    make_move(
        "Go Airborne",
        &[Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Flight,
                stacks: flight,
            },
            id_source: None,
            target: TARGET_SOURCE,
        }],
        Intent::Unknown,
    )
}

static MOVE_PECK_5: Move = make_move_attack("Peck", 1, 5);
static MOVE_PECK_6: Move = make_move_attack("Peck", 1, 6);
static MOVE_SWOOP_12: Move = make_move_attack("Swoop", 12, 1);
static MOVE_SWOOP_14: Move = make_move_attack("Swoop", 14, 1);
static MOVE_CAW: Move = make_move_buff("Caw", ModifierKind::Strength, 1);
static MOVE_STUNNED: Move = make_move("Stunned", &[], Intent::Stunned);
static MOVE_HEADBUTT: Move = make_move_attack("Headbutt", 3, 1);
static MOVE_GO_AIRBORNE_3: Move = make_move_go_airborne(3);
static MOVE_GO_AIRBORNE_4: Move = make_move_go_airborne(4);

static MOVES_ASC0: [Move; 6] = [
    MOVE_PECK_5,
    MOVE_SWOOP_12,
    MOVE_CAW,
    MOVE_STUNNED,
    MOVE_HEADBUTT,
    MOVE_GO_AIRBORNE_3,
];
static MOVES_ASC2: [Move; 6] = [
    MOVE_PECK_6,
    MOVE_SWOOP_14,
    MOVE_CAW,
    MOVE_STUNNED,
    MOVE_HEADBUTT,
    MOVE_GO_AIRBORNE_3,
];
static MOVES_ASC17: [Move; 6] = [
    MOVE_PECK_6,
    MOVE_SWOOP_14,
    MOVE_CAW,
    MOVE_STUNNED,
    MOVE_HEADBUTT,
    MOVE_GO_AIRBORNE_4,
];

const IDX_MOVE_PECK: usize = 0;
const IDX_MOVE_SWOOP: usize = 1;
const IDX_MOVE_CAW: usize = 2;
pub const IDX_MOVE_STUNNED: usize = 3;
const IDX_MOVE_HEADBUTT: usize = 4;
const IDX_MOVE_GO_AIRBORNE: usize = 5;

pub fn flight_stacks(ascension_level: u8) -> i16 {
    if ascension_level < 17 { 3 } else { 4 }
}

pub fn spawn_monster_byrd(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (25, 31)
    } else {
        (26, 33)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(
        &mut modifiers,
        ModifierKind::Flight,
        flight_stacks(ascension_level),
    );

    make_entity_monster(
        MonsterName::Byrd,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}

pub fn get_next_move_byrd(
    move_current: Option<usize>,
    move_history: &[u8],
    modifiers: &Modifiers,
    rng: &mut impl Rng,
) -> usize {
    let flying = has_modifier(modifiers, ModifierKind::Flight);
    if move_current.is_none() {
        return if rng.random_bool(0.375) {
            IDX_MOVE_CAW
        } else {
            IDX_MOVE_PECK
        };
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    // Grounded cycle: Stunned -> Headbutt -> Go Airborne -> (flying again)
    if !flying {
        return match last {
            IDX_MOVE_STUNNED => IDX_MOVE_HEADBUTT,
            IDX_MOVE_HEADBUTT => IDX_MOVE_GO_AIRBORNE,
            _ => unreachable!("Byrd grounded with unexpected last move: {last}"),
        };
    }

    let roll = rng.random_range(0..=99);
    if roll < 50 {
        if move_history.ends_with(&[IDX_MOVE_PECK as u8, IDX_MOVE_PECK as u8]) {
            if rng.random_bool(0.4) {
                IDX_MOVE_SWOOP
            } else {
                IDX_MOVE_CAW
            }
        } else {
            IDX_MOVE_PECK
        }
    } else if roll < 70 {
        if last == IDX_MOVE_SWOOP {
            if rng.random_bool(0.375) {
                IDX_MOVE_CAW
            } else {
                IDX_MOVE_PECK
            }
        } else {
            IDX_MOVE_SWOOP
        }
    } else if last == IDX_MOVE_CAW {
        if rng.random_bool(0.2857) {
            IDX_MOVE_SWOOP
        } else {
            IDX_MOVE_PECK
        }
    } else {
        IDX_MOVE_CAW
    }
}
