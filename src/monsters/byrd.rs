use crate::effect::Effect;
use crate::effect::EffectKind;
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
use crate::monsters::move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

const fn move_go_airborne(flight: i16) -> Move {
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

static MOVE_PECK_5: Move = move_attack("Peck", 1, 5);
static MOVE_PECK_6: Move = move_attack("Peck", 1, 6);
static MOVE_SWOOP_12: Move = move_attack("Swoop", 12, 1);
static MOVE_SWOOP_14: Move = move_attack("Swoop", 14, 1);
static MOVE_CAW: Move = move_buff("Caw", ModifierKind::Strength, 1);
static MOVE_STUNNED: Move = make_move("Stunned", &[], Intent::Stunned);
static MOVE_HEADBUTT: Move = move_attack("Headbutt", 3, 1);
static MOVE_GO_AIRBORNE_3: Move = move_go_airborne(3);
static MOVE_GO_AIRBORNE_4: Move = move_go_airborne(4);

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

const FLIGHT_STACKS_BASE: i16 = 3;
const FLIGHT_STACKS_A17: i16 = 4;

// Turn start re-reads this to reset Flight
pub fn flight_stacks(ascension_level: u8) -> i16 {
    if ascension_level < 17 {
        FLIGHT_STACKS_BASE
    } else {
        FLIGHT_STACKS_A17
    }
}

pub static BYRD: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Byrd,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (25, 31)), (7, (26, 33))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[
        (
            0,
            &[modifier_fixed(ModifierKind::Flight, FLIGHT_STACKS_BASE)],
        ),
        (
            17,
            &[modifier_fixed(ModifierKind::Flight, FLIGHT_STACKS_A17)],
        ),
    ],
};

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
