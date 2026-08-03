use crate::entity::Entity;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::entity::make_move_attack;
use crate::entity::make_move_attack_debuff;
use crate::entity::make_move_buff;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_BELLOW_2: Move = make_move_buff("Bellow", ModifierKind::Enrage, 2);
static MOVE_BELLOW_3: Move = make_move_buff("Bellow", ModifierKind::Enrage, 3);
static MOVE_BULL_RUSH_14: Move = make_move_attack("Bull Rush", 14, 1);
static MOVE_BULL_RUSH_16: Move = make_move_attack("Bull Rush", 16, 1);
static MOVE_SKULL_BASH_6: Move =
    make_move_attack_debuff("Skull Bash", 6, ModifierKind::Vulnerable, 2);
static MOVE_SKULL_BASH_8: Move =
    make_move_attack_debuff("Skull Bash", 8, ModifierKind::Vulnerable, 2);

static MOVES_ASC0: [Move; 3] = [MOVE_BELLOW_2, MOVE_BULL_RUSH_14, MOVE_SKULL_BASH_6];
static MOVES_ASC3: [Move; 3] = [MOVE_BELLOW_2, MOVE_BULL_RUSH_16, MOVE_SKULL_BASH_8];
static MOVES_ASC18: [Move; 3] = [MOVE_BELLOW_3, MOVE_BULL_RUSH_16, MOVE_SKULL_BASH_8];

const IDX_MOVE_BELLOW: usize = 0;
const IDX_MOVE_BULL_RUSH: usize = 1;
const IDX_MOVE_SKULL_BASH: usize = 2;

pub fn spawn_monster_gremlin_nob(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (82, 86)
    } else {
        (85, 90)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 3 {
        &MOVES_ASC0
    } else if ascension_level < 18 {
        &MOVES_ASC3
    } else {
        &MOVES_ASC18
    };

    make_entity_monster(
        MonsterName::GremlinNob,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

pub fn get_next_move_gremlin_nob(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    // First turn: always Bellow
    let bellow_used = move_history.iter().any(|&m| m == IDX_MOVE_BELLOW as u8);
    if !bellow_used {
        return IDX_MOVE_BELLOW;
    }

    if ascension_level >= 18 {
        // Skull Bash if neither of the last two moves was Skull Bash
        let last = move_history.last().copied();
        let last_before = if move_history.len() >= 2 {
            Some(move_history[move_history.len() - 2])
        } else {
            None
        };
        let recent_skull_bash = last == Some(IDX_MOVE_SKULL_BASH as u8)
            || last_before == Some(IDX_MOVE_SKULL_BASH as u8);
        if !recent_skull_bash {
            return IDX_MOVE_SKULL_BASH;
        }
        IDX_MOVE_BULL_RUSH
    } else {
        // Asc 0–17: 33% Skull Bash; else Bull Rush with no-3-in-a-row constraint
        let roll = rng.random_range(0..=99);
        if roll < 33 {
            return IDX_MOVE_SKULL_BASH;
        }
        if move_history.ends_with(&[IDX_MOVE_BULL_RUSH as u8, IDX_MOVE_BULL_RUSH as u8]) {
            IDX_MOVE_SKULL_BASH
        } else {
            IDX_MOVE_BULL_RUSH
        }
    }
}
