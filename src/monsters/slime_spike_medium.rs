use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move_attack_card_add;
use crate::monsters::make_move_debuff;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_FLAME_TACKLE_8: Move =
    make_move_attack_card_add("Flame Tackle", 8, CardName::Slimed, 1, false);
static MOVE_FLAME_TACKLE_10: Move =
    make_move_attack_card_add("Flame Tackle", 10, CardName::Slimed, 1, false);
static MOVE_LICK: Move = make_move_debuff("Lick", ModifierKind::Frail, 1, Intent::Debuff);

static MOVES_ASC0: [Move; 2] = [MOVE_FLAME_TACKLE_8, MOVE_LICK];
static MOVES_ASC2: [Move; 2] = [MOVE_FLAME_TACKLE_10, MOVE_LICK];

const IDX_MOVE_FLAME_TACKLE: usize = 0;
const IDX_MOVE_LICK: usize = 1;

pub fn spawn_monster_slime_spike_medium(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (28, 32)
    } else {
        (29, 34)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    make_entity_monster(
        MonsterName::SlimeSpikeMedium,
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

pub fn get_next_move_slime_spike(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        if roll < 30 {
            // Flame Tackle: Asc 17+ no-three-in-a-row -> fall back to Lick
            if move_history.ends_with(&[IDX_MOVE_FLAME_TACKLE as u8, IDX_MOVE_FLAME_TACKLE as u8]) {
                IDX_MOVE_LICK
            } else {
                IDX_MOVE_FLAME_TACKLE
            }
        } else if move_history.last().copied() == Some(IDX_MOVE_LICK as u8) {
            // Lick: Asc 17+ no-two-in-a-row -> fall back to Flame Tackle
            IDX_MOVE_FLAME_TACKLE
        } else {
            IDX_MOVE_LICK
        }
    } else if roll < 30 {
        // Flame Tackle: Asc 0-16 no-three-in-a-row -> fall back to Lick
        if move_history.ends_with(&[IDX_MOVE_FLAME_TACKLE as u8, IDX_MOVE_FLAME_TACKLE as u8]) {
            IDX_MOVE_LICK
        } else {
            IDX_MOVE_FLAME_TACKLE
        }
    } else if move_history.ends_with(&[IDX_MOVE_LICK as u8, IDX_MOVE_LICK as u8]) {
        // Lick: Asc 0-16 no-three-in-a-row -> fall back to Flame Tackle
        IDX_MOVE_FLAME_TACKLE
    } else {
        IDX_MOVE_LICK
    }
}
