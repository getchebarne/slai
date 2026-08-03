use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::entity::make_move;
use crate::entity::make_move_attack;
use crate::modifier::ZERO_MODIFIERS;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_CHARGE: Move = make_move("Charge", &[], Intent::Unknown);
static MOVE_ULTIMATE_BLAST_25: Move = make_move_attack("Ultimate Blast", 25, 1);
static MOVE_ULTIMATE_BLAST_30: Move = make_move_attack("Ultimate Blast", 30, 1);
static MOVES_ASC0: [Move; 2] = [MOVE_CHARGE, MOVE_ULTIMATE_BLAST_25];
static MOVES_ASC2: [Move; 2] = [MOVE_CHARGE, MOVE_ULTIMATE_BLAST_30];
static MOVES_ASC17: [Move; 2] = [MOVE_CHARGE, MOVE_ULTIMATE_BLAST_30];

const IDX_MOVE_CHARGE: usize = 0;
const IDX_MOVE_ULTIMATE_BLAST: usize = 1;

pub fn spawn_monster_gremlin_wizard(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (21, 25)
    } else {
        (22, 26)
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
        MonsterName::GremlinWizard,
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

pub fn get_next_move_gremlin_wizard(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_CHARGE;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    if last == IDX_MOVE_ULTIMATE_BLAST {
        return if ascension_level >= 17 {
            IDX_MOVE_ULTIMATE_BLAST
        } else {
            IDX_MOVE_CHARGE
        };
    }

    // Charges needed depends on whether Ultimate Blast has already fired
    let has_fired_blast = move_history
        .iter()
        .any(|&m| m == IDX_MOVE_ULTIMATE_BLAST as u8);

    let charges_needed = if has_fired_blast { 3 } else { 2 };

    // Get number of trialing charges
    let trailing_charges = move_history
        .iter()
        .rev()
        .take_while(|&&m| m == IDX_MOVE_CHARGE as u8)
        .count();

    if trailing_charges >= charges_needed {
        IDX_MOVE_ULTIMATE_BLAST
    } else {
        IDX_MOVE_CHARGE
    }
}
