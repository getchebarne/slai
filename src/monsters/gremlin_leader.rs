use crate::consts::MAX_MONSTERS;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Rally: two summons off the weighted gremlin pool
static MOVE_RALLY: Move = make_move(
    "Rally!",
    &[
        Effect {
            kind: EffectKind::GremlinSummon,
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::GremlinSummon,
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    Intent::Unknown,
);

// Encourage: Strength to everyone, block to the others
const fn make_move_encourage(strength: i16, block: u16) -> Move {
    make_move(
        "Encourage",
        &[
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: strength,
                },
                id_source: None,
                target: TARGET_MONSTERS_ALL,
            },
            Effect {
                kind: EffectKind::BlockGain { amount: block },
                id_source: None,
                target: Target::Resolve {
                    candidate_pool: CandidatePool::Monsters,
                    filter: CandidateFilter::NotSource,
                    selection_kind: SelectionKind::All,
                },
            },
        ],
        Intent::BlockBuff,
    )
}

static MOVE_ENCOURAGE_3: Move = make_move_encourage(3, 6);
static MOVE_ENCOURAGE_4: Move = make_move_encourage(4, 6);
static MOVE_ENCOURAGE_5: Move = make_move_encourage(5, 10);
static MOVE_STAB: Move = make_move_attack("Stab", 6, 3);

static MOVES_ASC0: [Move; 3] = [MOVE_RALLY, MOVE_ENCOURAGE_3, MOVE_STAB];
static MOVES_ASC3: [Move; 3] = [MOVE_RALLY, MOVE_ENCOURAGE_4, MOVE_STAB];
static MOVES_ASC18: [Move; 3] = [MOVE_RALLY, MOVE_ENCOURAGE_5, MOVE_STAB];

const IDX_MOVE_RALLY: usize = 0;
const IDX_MOVE_ENCOURAGE: usize = 1;
const IDX_MOVE_STAB: usize = 2;

pub fn spawn_monster_gremlin_leader(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (140, 148)
    } else {
        (145, 155)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 3 {
        &MOVES_ASC0
    } else if ascension_level < 18 {
        &MOVES_ASC3
    } else {
        &MOVES_ASC18
    };

    make_entity_monster(
        MonsterName::GremlinLeader,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        MODIFIERS_ZERO,
        moves,
    )
}

pub fn get_next_move_gremlin_leader(
    move_history: &[u8],
    entity_id: usize,
    id_monsters: &[Option<usize>; MAX_MONSTERS],
    rng: &mut impl Rng,
) -> usize {
    let gremlins_alive = id_monsters
        .iter()
        .flatten()
        .filter(|&&id| id != entity_id)
        .count();
    let last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    let roll = rng.random_range(0..=99);

    if gremlins_alive == 1 {
        // A repeat-blocked band re-rolls into the complementary range (source recursion)
        let roll = if roll < 50 {
            if last != Some(IDX_MOVE_RALLY) {
                return IDX_MOVE_RALLY;
            }
            rng.random_range(50..=99)
        } else {
            roll
        };
        return if roll < 80 {
            if last == Some(IDX_MOVE_ENCOURAGE) {
                IDX_MOVE_STAB
            } else {
                IDX_MOVE_ENCOURAGE
            }
        } else if last == Some(IDX_MOVE_STAB) {
            if rng.random_range(0..=79) < 50 {
                IDX_MOVE_RALLY
            } else {
                IDX_MOVE_ENCOURAGE
            }
        } else {
            IDX_MOVE_STAB
        };
    }

    // Alone the Leader favors Rally; with a full posse, Encourage
    let (threshold, primary) = if gremlins_alive == 0 {
        (75, IDX_MOVE_RALLY)
    } else {
        (66, IDX_MOVE_ENCOURAGE)
    };
    if roll < threshold {
        if last == Some(primary) {
            IDX_MOVE_STAB
        } else {
            primary
        }
    } else if last == Some(IDX_MOVE_STAB) {
        primary
    } else {
        IDX_MOVE_STAB
    }
}
