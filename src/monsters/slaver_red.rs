use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_attack_debuff;
use crate::monsters::make_move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_STAB_13: Move = make_move_attack("Stab", 13, 1);
static MOVE_STAB_14: Move = make_move_attack("Stab", 14, 1);
static MOVE_ENTANGLE: Move = make_move_debuff(
    "Entangle",
    ModifierKind::Entangled,
    1,
    Intent::DebuffPowerful,
);
static MOVE_SCRAPE_8_VULN_1: Move =
    make_move_attack_debuff("Scrape", 8, ModifierKind::Vulnerable, 1);
static MOVE_SCRAPE_9_VULN_1: Move =
    make_move_attack_debuff("Scrape", 9, ModifierKind::Vulnerable, 1);
static MOVE_SCRAPE_9_VULN_2: Move =
    make_move_attack_debuff("Scrape", 9, ModifierKind::Vulnerable, 2);

static MOVES_ASC0: [Move; 3] = [MOVE_STAB_13, MOVE_ENTANGLE, MOVE_SCRAPE_8_VULN_1];
static MOVES_ASC2: [Move; 3] = [MOVE_STAB_14, MOVE_ENTANGLE, MOVE_SCRAPE_9_VULN_1];
static MOVES_ASC17: [Move; 3] = [MOVE_STAB_14, MOVE_ENTANGLE, MOVE_SCRAPE_9_VULN_2];

const IDX_MOVE_STAB: usize = 0;
const IDX_MOVE_ENTANGLE: usize = 1;
const IDX_MOVE_SCRAPE: usize = 2;

pub fn spawn_monster_slaver_red(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (46, 50)
    } else {
        (48, 52)
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
        MonsterName::SlaverRed,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        MODIFIERS_ZERO,
        moves,
    )
}

pub fn get_next_move_slaver_red(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_STAB;
    }
    let used_entangle = move_history.iter().any(|&m| m == IDX_MOVE_ENTANGLE as u8);

    let roll = rng.random_range(0..=99);
    let last_two_stab = move_history.ends_with(&[IDX_MOVE_STAB as u8, IDX_MOVE_STAB as u8]);

    if roll >= 75 && !used_entangle {
        return IDX_MOVE_ENTANGLE;
    }
    if roll >= 55 && used_entangle && !last_two_stab {
        return IDX_MOVE_STAB;
    }
    if ascension_level >= 17 {
        if move_history.last().copied() != Some(IDX_MOVE_SCRAPE as u8) {
            IDX_MOVE_SCRAPE
        } else {
            IDX_MOVE_STAB
        }
    } else if !move_history.ends_with(&[IDX_MOVE_SCRAPE as u8, IDX_MOVE_SCRAPE as u8]) {
        IDX_MOVE_SCRAPE
    } else {
        IDX_MOVE_STAB
    }
}
