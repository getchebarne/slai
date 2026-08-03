use crate::entity::Entity;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::entity::make_move_attack;
use crate::entity::make_move_buff;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_BITE_5: Move = make_move_attack("Bite", 5, 1);
static MOVE_BITE_6: Move = make_move_attack("Bite", 6, 1);
static MOVE_BITE_7: Move = make_move_attack("Bite", 7, 1);
static MOVE_BITE_8: Move = make_move_attack("Bite", 8, 1);
static MOVE_STRENGTHEN_3: Move = make_move_buff("Grow", ModifierKind::Strength, 3);
static MOVE_STRENGTHEN_4: Move = make_move_buff("Grow", ModifierKind::Strength, 4);

// 9 move tables: 3 asc brackets x 3 bite values (5/6/7 at Asc 0-1, 6/7/8 at Asc 2+)
static MOVES_ASC0_BITE5: [Move; 2] = [MOVE_BITE_5, MOVE_STRENGTHEN_3];
static MOVES_ASC0_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_3];
static MOVES_ASC0_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_3];
static MOVES_ASC2_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_3];
static MOVES_ASC2_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_3];
static MOVES_ASC2_BITE8: [Move; 2] = [MOVE_BITE_8, MOVE_STRENGTHEN_3];
static MOVES_ASC17_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_4];
static MOVES_ASC17_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_4];
static MOVES_ASC17_BITE8: [Move; 2] = [MOVE_BITE_8, MOVE_STRENGTHEN_4];

const IDX_MOVE_BITE: usize = 0;
const IDX_MOVE_STRENGTHEN: usize = 1;

pub fn spawn_monster_louse_red(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (10, 15)
    } else {
        (11, 16)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let bite_dmg: u8 = if ascension_level < 2 {
        rng.random_range(5..=7)
    } else {
        rng.random_range(6..=8)
    };

    let moves: &'static [Move] = if ascension_level < 2 {
        match bite_dmg {
            5 => &MOVES_ASC0_BITE5,
            6 => &MOVES_ASC0_BITE6,
            7 => &MOVES_ASC0_BITE7,
            _ => unreachable!("Asc 0–1 bite damage must be in 5..=7"),
        }
    } else if ascension_level < 17 {
        match bite_dmg {
            6 => &MOVES_ASC2_BITE6,
            7 => &MOVES_ASC2_BITE7,
            8 => &MOVES_ASC2_BITE8,
            _ => unreachable!("Asc 2–16 bite damage must be in 6..=8"),
        }
    } else {
        match bite_dmg {
            6 => &MOVES_ASC17_BITE6,
            7 => &MOVES_ASC17_BITE7,
            8 => &MOVES_ASC17_BITE8,
            _ => unreachable!("Asc 17+ bite damage must be in 6..=8"),
        }
    };

    let curl_up_stacks: i16 = if ascension_level < 7 {
        rng.random_range(3..=7)
    } else if ascension_level < 17 {
        rng.random_range(4..=8)
    } else {
        rng.random_range(9..=12)
    };
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::CurlUp, curl_up_stacks);

    make_entity_monster(
        MonsterName::LouseNormal,
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
