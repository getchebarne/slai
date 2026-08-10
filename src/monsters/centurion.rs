use crate::consts::MAX_MONSTERS;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Defend: block onto a random other Monster
const fn make_move_defend(block: u16) -> Move {
    make_move(
        "Defend",
        &[Effect {
            kind: EffectKind::BlockGain { amount: block },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Monsters,
                filter: CandidateFilter::NotSource,
                selection_kind: SelectionKind::Random { count: 1 },
            },
        }],
        Intent::Block,
    )
}

static MOVE_SLASH_12: Move = make_move_attack("Slash", 12, 1);
static MOVE_SLASH_14: Move = make_move_attack("Slash", 14, 1);
static MOVE_FURY_6: Move = make_move_attack("Fury", 6, 3);
static MOVE_FURY_7: Move = make_move_attack("Fury", 7, 3);
static MOVE_DEFEND_15: Move = make_move_defend(15);
static MOVE_DEFEND_20: Move = make_move_defend(20);

static MOVES_ASC0: [Move; 3] = [MOVE_SLASH_12, MOVE_FURY_6, MOVE_DEFEND_15];
static MOVES_ASC2: [Move; 3] = [MOVE_SLASH_14, MOVE_FURY_7, MOVE_DEFEND_15];
static MOVES_ASC17: [Move; 3] = [MOVE_SLASH_14, MOVE_FURY_7, MOVE_DEFEND_20];

const IDX_MOVE_SLASH: usize = 0;
const IDX_MOVE_FURY: usize = 1;
const IDX_MOVE_DEFEND: usize = 2;

pub fn spawn_monster_centurion(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (76, 80)
    } else {
        (78, 83)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    make_entity_monster(
        MonsterName::Centurion,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

pub fn get_next_move_centurion(
    move_history: &[u8],
    entity_id: usize,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    rng: &mut impl Rng,
) -> usize {
    // Fury only ever fires once the Centurion stands alone
    let alone = !id_monsters.iter().flatten().any(|&id| id != entity_id);
    let defend_or_fury = if alone {
        IDX_MOVE_FURY
    } else {
        IDX_MOVE_DEFEND
    };

    let roll = rng.random_range(0..=99);
    if (roll >= 65
        && !move_history.ends_with(&[IDX_MOVE_DEFEND as u8, IDX_MOVE_DEFEND as u8])
        && !move_history.ends_with(&[IDX_MOVE_FURY as u8, IDX_MOVE_FURY as u8]))
        || move_history.ends_with(&[IDX_MOVE_SLASH as u8, IDX_MOVE_SLASH as u8])
    {
        defend_or_fury
    } else {
        IDX_MOVE_SLASH
    }
}
