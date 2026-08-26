use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_attack_card_add;
use crate::monsters::move_debuff;
use crate::monsters::move_split;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_SLIME_TACKLE_11: Move =
    move_attack_card_add("Corrosive Spit", 11, CardName::Slimed, 2, false);
static MOVE_SLIME_TACKLE_12: Move =
    move_attack_card_add("Corrosive Spit", 12, CardName::Slimed, 2, false);
static MOVE_HEAVY_TACKLE_16: Move = move_attack("Tackle", 16, 1);
static MOVE_HEAVY_TACKLE_18: Move = move_attack("Tackle", 18, 1);
static MOVE_LICK: Move = move_debuff("Lick", ModifierKind::Weak, 2, Intent::Debuff);
static MOVE_SPLIT: Move = move_split(
    "Split",
    MonsterName::SlimeAcidMedium,
    MonsterName::SlimeAcidMedium,
);

static MOVES_ASC0: [Move; 4] = [
    MOVE_SLIME_TACKLE_11,
    MOVE_HEAVY_TACKLE_16,
    MOVE_LICK,
    MOVE_SPLIT,
];
static MOVES_ASC2: [Move; 4] = [
    MOVE_SLIME_TACKLE_12,
    MOVE_HEAVY_TACKLE_18,
    MOVE_LICK,
    MOVE_SPLIT,
];

const IDX_MOVE_SLIME_TACKLE: usize = 0;
const IDX_MOVE_HEAVY_TACKLE: usize = 1;
const IDX_MOVE_LICK: usize = 2;
pub const IDX_MOVE_SPLIT: usize = 3;

pub static SLIME_ACID_LARGE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlimeAcidLarge,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (65, 69)), (7, (68, 72))],
    block_start: 0,
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Splittable, 1)])],
};

pub fn get_next_move_slime_acid_large(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        // Asc 17+ 40/30/30: Tackle no-3-row, Heavy no-3-row, Lick no-2-row
        if roll < 40 {
            if move_history.ends_with(&[IDX_MOVE_SLIME_TACKLE as u8, IDX_MOVE_SLIME_TACKLE as u8]) {
                if rng.random_bool(0.6) {
                    IDX_MOVE_HEAVY_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_SLIME_TACKLE
            }
        } else if roll < 70 {
            if move_history.ends_with(&[IDX_MOVE_HEAVY_TACKLE as u8, IDX_MOVE_HEAVY_TACKLE as u8]) {
                if rng.random_bool(0.6) {
                    IDX_MOVE_SLIME_TACKLE
                } else {
                    IDX_MOVE_LICK
                }
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else if move_history.last().copied() == Some(IDX_MOVE_LICK as u8) {
            if rng.random_bool(0.4) {
                IDX_MOVE_SLIME_TACKLE
            } else {
                IDX_MOVE_HEAVY_TACKLE
            }
        } else {
            IDX_MOVE_LICK
        }
    } else if roll < 30 {
        // Asc 0-16 30/40/30: Tackle no-3-row, Heavy no-2-row, Lick no-3-row
        if move_history.ends_with(&[IDX_MOVE_SLIME_TACKLE as u8, IDX_MOVE_SLIME_TACKLE as u8]) {
            if rng.random_bool(0.5) {
                IDX_MOVE_HEAVY_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_SLIME_TACKLE
        }
    } else if roll < 70 {
        if move_history.last().copied() == Some(IDX_MOVE_HEAVY_TACKLE as u8) {
            if rng.random_bool(0.4) {
                IDX_MOVE_SLIME_TACKLE
            } else {
                IDX_MOVE_LICK
            }
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else if move_history.ends_with(&[IDX_MOVE_LICK as u8, IDX_MOVE_LICK as u8]) {
        if rng.random_bool(0.4) {
            IDX_MOVE_SLIME_TACKLE
        } else {
            IDX_MOVE_HEAVY_TACKLE
        }
    } else {
        IDX_MOVE_LICK
    }
}
