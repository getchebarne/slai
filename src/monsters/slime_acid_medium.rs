use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack;
use crate::monsters::move_attack_card_add;
use crate::monsters::move_debuff;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_WOUND_TACKLE_7: Move =
    move_attack_card_add("Corrosive Spit", 7, CardName::Slimed, 1, false);
static MOVE_WOUND_TACKLE_8: Move =
    move_attack_card_add("Corrosive Spit", 8, CardName::Slimed, 1, false);
static MOVE_HEAVY_TACKLE_10: Move = move_attack("Tackle", 10, 1);
static MOVE_HEAVY_TACKLE_12: Move = move_attack("Tackle", 12, 1);
static MOVE_LICK: Move = move_debuff("Lick", ModifierKind::Weak, 1, Intent::Debuff);

static MOVES_ASC0: [Move; 3] = [MOVE_WOUND_TACKLE_7, MOVE_HEAVY_TACKLE_10, MOVE_LICK];
static MOVES_ASC2: [Move; 3] = [MOVE_WOUND_TACKLE_8, MOVE_HEAVY_TACKLE_12, MOVE_LICK];

const IDX_MOVE_WOUND_TACKLE: usize = 0;
const IDX_MOVE_HEAVY_TACKLE: usize = 1;
const IDX_MOVE_LICK: usize = 2;

pub static TEMPLATE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlimeAcidMedium,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (28, 32)), (7, (29, 34))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2)],
    modifier_tiers: &[],
};

pub fn get_next_move_slime_acid_medium(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        // Asc 17+: 40/40/20 split with stricter constraints
        if roll < 40 {
            // Prefer Wound Tackle, but not three in a row
            if move_history.ends_with(&[IDX_MOVE_WOUND_TACKLE as u8, IDX_MOVE_WOUND_TACKLE as u8]) {
                if rng.random_bool(0.5) {
                    IDX_MOVE_HEAVY_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_WOUND_TACKLE
            }
        } else if roll < 80 {
            // Prefer Heavy Tackle, but not three in a row
            if move_history.ends_with(&[IDX_MOVE_HEAVY_TACKLE as u8, IDX_MOVE_HEAVY_TACKLE as u8]) {
                if rng.random_bool(0.5) {
                    IDX_MOVE_WOUND_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else if move_history.last().copied() == Some(IDX_MOVE_LICK as u8) {
            // Lick: Asc 17+ no-two-in-a-row
            if rng.random_bool(0.4) {
                IDX_MOVE_WOUND_TACKLE
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else {
            IDX_MOVE_LICK
        }
    } else if roll < 30 {
        // Wound Tackle: Asc 0-16 no-three-in-a-row
        if move_history.ends_with(&[IDX_MOVE_WOUND_TACKLE as u8, IDX_MOVE_WOUND_TACKLE as u8]) {
            if rng.random_bool(0.5) {
                IDX_MOVE_HEAVY_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_WOUND_TACKLE
        }
    } else if roll < 70 {
        // Heavy Tackle: Asc 0-16 no-two-in-a-row (looser than Asc 17+)
        if move_history.last().copied() == Some(IDX_MOVE_HEAVY_TACKLE as u8) {
            if rng.random_bool(0.4) {
                IDX_MOVE_WOUND_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else if move_history.ends_with(&[IDX_MOVE_LICK as u8, IDX_MOVE_LICK as u8]) {
        // Lick: Asc 0-16 no-three-in-a-row.
        if rng.random_bool(0.4) {
            IDX_MOVE_WOUND_TACKLE
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else {
        IDX_MOVE_LICK
    }
}
