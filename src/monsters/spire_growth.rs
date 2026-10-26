use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::monsters::move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_QUICK_TACKLE_16: Move = move_attack("Quick Tackle", 16, 1);
static MOVE_QUICK_TACKLE_18: Move = move_attack("Quick Tackle", 18, 1);
static MOVE_CONSTRICT_10: Move = move_debuff(
    "Constrict",
    ModifierKind::Constricted,
    10,
    Intent::DebuffPowerful,
);
static MOVE_CONSTRICT_12: Move = move_debuff(
    "Constrict",
    ModifierKind::Constricted,
    12,
    Intent::DebuffPowerful,
);
static MOVE_SMASH_22: Move = move_attack("Smash", 22, 1);
static MOVE_SMASH_25: Move = move_attack("Smash", 25, 1);
static MOVES_ASC0: [Move; 3] = [MOVE_QUICK_TACKLE_16, MOVE_CONSTRICT_10, MOVE_SMASH_22];
static MOVES_ASC2: [Move; 3] = [MOVE_QUICK_TACKLE_18, MOVE_CONSTRICT_10, MOVE_SMASH_25];
static MOVES_ASC17: [Move; 3] = [MOVE_QUICK_TACKLE_18, MOVE_CONSTRICT_12, MOVE_SMASH_25];

const IDX_MOVE_QUICK_TACKLE: usize = 0;
const IDX_MOVE_CONSTRICT: usize = 1;
const IDX_MOVE_SMASH: usize = 2;

pub static SPIRE_GROWTH: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SpireGrowth,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (170, 170)), (7, (190, 190))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (2, &[&MOVES_ASC2]),
        (17, &[&MOVES_ASC17]),
    ],
    modifier_tiers: &[],
};

// Every roll draws, even when Asc 17's Constrict decides without it; the first move rolls like any other, on an empty history
pub fn get_next_move_spire_growth(
    move_history: &[u8],
    character_constricted: bool,
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    let constrict_last = move_history.last().copied() == Some(IDX_MOVE_CONSTRICT as u8);
    let constrict_allowed = !character_constricted && !constrict_last;

    if ascension_level >= 17 && constrict_allowed {
        return IDX_MOVE_CONSTRICT;
    }
    let tackle_twice =
        move_history.ends_with(&[IDX_MOVE_QUICK_TACKLE as u8, IDX_MOVE_QUICK_TACKLE as u8]);
    let smash_twice = move_history.ends_with(&[IDX_MOVE_SMASH as u8, IDX_MOVE_SMASH as u8]);
    if roll < 50 && !tackle_twice {
        IDX_MOVE_QUICK_TACKLE
    } else if constrict_allowed {
        IDX_MOVE_CONSTRICT
    } else if !smash_twice {
        IDX_MOVE_SMASH
    } else {
        IDX_MOVE_QUICK_TACKLE
    }
}
