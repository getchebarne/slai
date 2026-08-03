use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::entity::make_move_attack;
use crate::entity::make_move_attack_card_add;
use crate::entity::make_move_debuff;
use crate::entity::make_move_split;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_SLIME_TACKLE_11: Move =
    make_move_attack_card_add("Corrosive Spit", 11, CardName::Slimed, 2, false);
static MOVE_SLIME_TACKLE_12: Move =
    make_move_attack_card_add("Corrosive Spit", 12, CardName::Slimed, 2, false);
static MOVE_HEAVY_TACKLE_16: Move = make_move_attack("Tackle", 16, 1);
static MOVE_HEAVY_TACKLE_18: Move = make_move_attack("Tackle", 18, 1);
static MOVE_LICK: Move = make_move_debuff("Lick", ModifierKind::Weak, 2, Intent::Debuff);
static MOVE_SPLIT: Move = make_move_split(
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

pub fn spawn_monster_slime_acid_large(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (65, 69)
    } else {
        (68, 72)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Splittable, 1);

    make_entity_monster(
        MonsterName::SlimeAcidLarge,
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
