use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::move_attack_card_add;
use crate::monsters::move_debuff;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_FLAME_TACKLE_8: Move =
    move_attack_card_add("Flame Tackle", 8, CardName::Slimed, 1, false);
static MOVE_FLAME_TACKLE_10: Move =
    move_attack_card_add("Flame Tackle", 10, CardName::Slimed, 1, false);
static MOVE_LICK: Move = move_debuff("Lick", ModifierKind::Frail, 1, Intent::Debuff);

static MOVES_ASC0: [Move; 2] = [MOVE_FLAME_TACKLE_8, MOVE_LICK];
static MOVES_ASC2: [Move; 2] = [MOVE_FLAME_TACKLE_10, MOVE_LICK];

const IDX_MOVE_FLAME_TACKLE: usize = 0;
const IDX_MOVE_LICK: usize = 1;

pub static SLIME_SPIKE_MEDIUM: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SlimeSpikeMedium,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (28, 32)), (7, (29, 34))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2)],
    modifier_tiers: &[],
};

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
