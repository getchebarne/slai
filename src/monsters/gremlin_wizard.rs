use crate::entity::Intent;
use crate::entity::Move;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_CHARGE: Move = make_move("Charge", &[], Intent::Unknown);
static MOVE_ULTIMATE_BLAST_25: Move = move_attack("Ultimate Blast", 25, 1);
static MOVE_ULTIMATE_BLAST_30: Move = move_attack("Ultimate Blast", 30, 1);
static MOVES_ASC0: [Move; 2] = [MOVE_CHARGE, MOVE_ULTIMATE_BLAST_25];
static MOVES_ASC2: [Move; 2] = [MOVE_CHARGE, MOVE_ULTIMATE_BLAST_30];
static MOVES_ASC17: [Move; 2] = [MOVE_CHARGE, MOVE_ULTIMATE_BLAST_30];

const IDX_MOVE_CHARGE: usize = 0;
const IDX_MOVE_ULTIMATE_BLAST: usize = 1;

pub static TEMPLATE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::GremlinWizard,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (21, 25)), (7, (22, 26))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

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
        .any(|&idx_move| idx_move == IDX_MOVE_ULTIMATE_BLAST as u8);

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
