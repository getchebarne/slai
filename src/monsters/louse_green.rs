use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_entity_monster;
use crate::monsters::move_attack;
use crate::monsters::move_debuff;
use crate::monsters::pick_tier;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_BITE_5: Move = move_attack("Bite", 5, 1);
static MOVE_BITE_6: Move = move_attack("Bite", 6, 1);
static MOVE_BITE_7: Move = move_attack("Bite", 7, 1);
static MOVE_BITE_8: Move = move_attack("Bite", 8, 1);
static MOVE_WEB: Move = move_debuff("Spit Web", ModifierKind::Weak, 2, Intent::Debuff);

static MOVES_ASC0_BITE5: [Move; 2] = [MOVE_BITE_5, MOVE_WEB];
static MOVES_ASC0_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_WEB];
static MOVES_ASC0_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_WEB];
static MOVES_ASC2_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_WEB];
static MOVES_ASC2_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_WEB];
static MOVES_ASC2_BITE8: [Move; 2] = [MOVE_BITE_8, MOVE_WEB];
static MOVES_ASC17_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_WEB];
static MOVES_ASC17_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_WEB];
static MOVES_ASC17_BITE8: [Move; 2] = [MOVE_BITE_8, MOVE_WEB];

const IDX_MOVE_BITE: usize = 0;
const IDX_MOVE_WEB: usize = 1;

// Moves and Curl Up are spawn-rolled (bite damage tables); see the spawn fn
pub static TEMPLATE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::LouseDefensive,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (11, 17)), (7, (12, 18))],
    block_start: 0,
    move_tiers: &[],
    modifier_tiers: &[],
};

pub fn spawn_monster_louse_green(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) =
        pick_tier(TEMPLATE.health_tiers, ascension_level).expect("health_tiers is never empty");
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
    let mut modifiers = MODIFIERS_ZERO;
    modifier_apply(&mut modifiers, ModifierKind::CurlUp, curl_up_stacks);

    make_entity_monster(
        MonsterName::LouseDefensive,
        TEMPLATE.kind,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}

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
