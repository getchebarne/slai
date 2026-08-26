use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_rolled;
use crate::monsters::move_attack;
use crate::monsters::move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_BITE_5: Move = move_attack("Bite", 5, 1);
static MOVE_BITE_6: Move = move_attack("Bite", 6, 1);
static MOVE_BITE_7: Move = move_attack("Bite", 7, 1);
static MOVE_BITE_8: Move = move_attack("Bite", 8, 1);
static MOVE_WEB: Move = move_debuff("Spit Web", ModifierKind::Weak, 2, Intent::Debuff);

static MOVES_BITE5: [Move; 2] = [MOVE_BITE_5, MOVE_WEB];
static MOVES_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_WEB];
static MOVES_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_WEB];
static MOVES_BITE8: [Move; 2] = [MOVE_BITE_8, MOVE_WEB];

// Bite damage is spawn-rolled; ascension only shifts the range
static MOVES_ASC0: [&[Move]; 3] = [&MOVES_BITE5, &MOVES_BITE6, &MOVES_BITE7];
static MOVES_ASC2: [&[Move]; 3] = [&MOVES_BITE6, &MOVES_BITE7, &MOVES_BITE8];

const IDX_MOVE_BITE: usize = 0;
const IDX_MOVE_WEB: usize = 1;

pub static LOUSE_DEFENSIVE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::LouseDefensive,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (11, 17)), (7, (12, 18))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2)],
    modifier_tiers: &[
        (0, &[modifier_rolled(ModifierKind::CurlUp, 3, 7)]),
        (7, &[modifier_rolled(ModifierKind::CurlUp, 4, 8)]),
        (17, &[modifier_rolled(ModifierKind::CurlUp, 9, 12)]),
    ],
};

pub fn get_next_move_louse_green(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        // Asc 17+: Web never twice in a row; Bite no constraint
        if roll < 25 {
            if move_history.last().copied() == Some(IDX_MOVE_WEB as u8) {
                IDX_MOVE_BITE
            } else {
                IDX_MOVE_WEB
            }
        } else if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_WEB
        } else {
            IDX_MOVE_BITE
        }
    } else {
        // Asc 0–16: Web no-two-in-a-row; Bite no-three-in-a-row
        if roll < 25 {
            if move_history.ends_with(&[IDX_MOVE_WEB as u8, IDX_MOVE_WEB as u8]) {
                IDX_MOVE_BITE
            } else {
                IDX_MOVE_WEB
            }
        } else if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_WEB
        } else {
            IDX_MOVE_BITE
        }
    }
}
