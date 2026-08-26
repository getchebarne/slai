use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::modifier_rolled;
use crate::monsters::move_attack;
use crate::monsters::move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_BITE_5: Move = move_attack("Bite", 5, 1);
static MOVE_BITE_6: Move = move_attack("Bite", 6, 1);
static MOVE_BITE_7: Move = move_attack("Bite", 7, 1);
static MOVE_BITE_8: Move = move_attack("Bite", 8, 1);
static MOVE_STRENGTHEN_3: Move = move_buff("Grow", ModifierKind::Strength, 3);
static MOVE_STRENGTHEN_4: Move = move_buff("Grow", ModifierKind::Strength, 4);

// 9 move tables: 3 asc brackets x 3 bite values (5/6/7 at Asc 0-1, 6/7/8 at Asc 2+)
static MOVES_BITE5_STR3: [Move; 2] = [MOVE_BITE_5, MOVE_STRENGTHEN_3];
static MOVES_BITE6_STR3: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_3];
static MOVES_BITE7_STR3: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_3];
static MOVES_BITE8_STR3: [Move; 2] = [MOVE_BITE_8, MOVE_STRENGTHEN_3];
static MOVES_BITE6_STR4: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_4];
static MOVES_BITE7_STR4: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_4];
static MOVES_BITE8_STR4: [Move; 2] = [MOVE_BITE_8, MOVE_STRENGTHEN_4];

// Bite damage is spawn-rolled; ascension shifts the range and, at 17+, Strengthen
static MOVES_ASC0: [&[Move]; 3] = [&MOVES_BITE5_STR3, &MOVES_BITE6_STR3, &MOVES_BITE7_STR3];
static MOVES_ASC2: [&[Move]; 3] = [&MOVES_BITE6_STR3, &MOVES_BITE7_STR3, &MOVES_BITE8_STR3];
static MOVES_ASC17: [&[Move]; 3] = [&MOVES_BITE6_STR4, &MOVES_BITE7_STR4, &MOVES_BITE8_STR4];

const IDX_MOVE_BITE: usize = 0;
const IDX_MOVE_STRENGTHEN: usize = 1;

// Moves and Curl Up are spawn-rolled (bite damage tables); see the spawn fn
pub static LOUSE_NORMAL: MonsterTemplate = MonsterTemplate {
    name: MonsterName::LouseNormal,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (10, 15)), (7, (11, 16))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[
        (0, &[modifier_rolled(ModifierKind::CurlUp, 3, 7)]),
        (7, &[modifier_rolled(ModifierKind::CurlUp, 4, 8)]),
        (17, &[modifier_rolled(ModifierKind::CurlUp, 9, 12)]),
    ],
};

pub fn get_next_move_louse_red(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        // Asc 17+: Strengthen never twice in a row; Bite no constraint
        if roll < 25 {
            if move_history.last().copied() == Some(IDX_MOVE_STRENGTHEN as u8) {
                IDX_MOVE_BITE
            } else {
                IDX_MOVE_STRENGTHEN
            }
        } else if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_STRENGTHEN
        } else {
            IDX_MOVE_BITE
        }
    } else {
        // Asc 0–16: Strengthen no-two-in-a-row; Bite no-three-in-a-row
        if roll < 25 {
            if move_history.ends_with(&[IDX_MOVE_STRENGTHEN as u8, IDX_MOVE_STRENGTHEN as u8]) {
                IDX_MOVE_BITE
            } else {
                IDX_MOVE_STRENGTHEN
            }
        } else if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_STRENGTHEN
        } else {
            IDX_MOVE_BITE
        }
    }
}
