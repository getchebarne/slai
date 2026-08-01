use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::entity::make_move;
use crate::modifier::ZERO_MODIFIERS;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_PUNCTURE_9: Move = make_move(
    "Puncture",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 9 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 9,
        instances: 1,
    },
);
static MOVE_PUNCTURE_10: Move = make_move(
    "Puncture",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 10 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 10,
        instances: 1,
    },
);
static MOVES_ASC0: [Move; 1] = [MOVE_PUNCTURE_9];
static MOVES_ASC2: [Move; 1] = [MOVE_PUNCTURE_10];
static MOVES_ASC17: [Move; 1] = [MOVE_PUNCTURE_10];

const IDX_MOVE_PUNCTURE: usize = 0;

pub fn spawn_monster_gremlin_thief(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (10, 14)
    } else {
        (11, 15)
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
        MonsterName::GremlinThief,
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

pub fn get_next_move_gremlin_thief() -> usize {
    IDX_MOVE_PUNCTURE
}
