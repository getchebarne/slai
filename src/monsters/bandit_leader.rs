use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_MOCK: Move = make_move("Mock", &[], Intent::Unknown);
static MOVE_AGONIZING_10_W2: Move =
    make_move_attack_debuff("Agonizing Slash", 10, ModifierKind::Weak, 2);
static MOVE_AGONIZING_12_W2: Move =
    make_move_attack_debuff("Agonizing Slash", 12, ModifierKind::Weak, 2);
static MOVE_AGONIZING_12_W3: Move =
    make_move_attack_debuff("Agonizing Slash", 12, ModifierKind::Weak, 3);
static MOVE_CROSS_SLASH_15: Move = make_move_attack("Cross Slash", 15, 1);
static MOVE_CROSS_SLASH_17: Move = make_move_attack("Cross Slash", 17, 1);

static MOVES_ASC0: [Move; 3] = [MOVE_MOCK, MOVE_AGONIZING_10_W2, MOVE_CROSS_SLASH_15];
static MOVES_ASC2: [Move; 3] = [MOVE_MOCK, MOVE_AGONIZING_12_W2, MOVE_CROSS_SLASH_17];
static MOVES_ASC17: [Move; 3] = [MOVE_MOCK, MOVE_AGONIZING_12_W3, MOVE_CROSS_SLASH_17];

const IDX_MOVE_MOCK: usize = 0;
const IDX_MOVE_AGONIZING: usize = 1;
const IDX_MOVE_CROSS_SLASH: usize = 2;

pub fn spawn_monster_bandit_leader(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (35, 39)
    } else {
        (37, 41)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    make_entity_monster(
        MonsterName::BanditLeader,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

// Mock opener, then Agonizing/Cross alternating; A17+ chains Cross Slash twice
pub fn get_next_move_bandit_leader(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_MOCK;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    match last {
        IDX_MOVE_MOCK => IDX_MOVE_AGONIZING,
        IDX_MOVE_AGONIZING => IDX_MOVE_CROSS_SLASH,
        IDX_MOVE_CROSS_SLASH => {
            if ascension_level >= 17
                && !move_history
                    .ends_with(&[IDX_MOVE_CROSS_SLASH as u8, IDX_MOVE_CROSS_SLASH as u8])
            {
                IDX_MOVE_CROSS_SLASH
            } else {
                IDX_MOVE_AGONIZING
            }
        }
        _ => unreachable!("Bandit Leader unexpected move idx: {last}"),
    }
}
