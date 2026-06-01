use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::modifier::ZERO_MODIFIERS;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_PROTECT_7: Move = Move {
    name: "Protect",
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 7 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters {
                filter: CandidatePoolMonstersFilter::Other,
            },
            selection_kind: SelectionKind::Random { count: 1 },
        },
    }],
    intent: Intent::Block,
};
static MOVE_PROTECT_8: Move = Move {
    name: "Protect",
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 8 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters {
                filter: CandidatePoolMonstersFilter::Other,
            },
            selection_kind: SelectionKind::Random { count: 1 },
        },
    }],
    intent: Intent::Block,
};
static MOVE_PROTECT_11: Move = Move {
    name: "Protect",
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 11 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters {
                filter: CandidatePoolMonstersFilter::Other,
            },
            selection_kind: SelectionKind::Random { count: 1 },
        },
    }],
    intent: Intent::Block,
};
static MOVE_BASH_6: Move = Move {
    name: "Shield Bash",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 6 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 6,
        instances: 1,
    },
};
static MOVE_BASH_8: Move = Move {
    name: "Shield Bash",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 8 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 8,
        instances: 1,
    },
};

static MOVES_ASC0: [Move; 2] = [MOVE_PROTECT_7, MOVE_BASH_6];
static MOVES_ASC2: [Move; 2] = [MOVE_PROTECT_7, MOVE_BASH_8];
static MOVES_ASC7: [Move; 2] = [MOVE_PROTECT_8, MOVE_BASH_8];
static MOVES_ASC17: [Move; 2] = [MOVE_PROTECT_11, MOVE_BASH_8];

const IDX_MOVE_PROTECT: usize = 0;
const IDX_MOVE_BASH: usize = 1;

pub fn spawn_monster_gremlin_tsundere(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (12, 15)
    } else {
        (13, 17)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 7 {
        &MOVES_ASC2
    } else if ascension_level < 17 {
        &MOVES_ASC7
    } else {
        &MOVES_ASC17
    };

    make_entity_monster(
        MonsterName::GremlinTsundere,
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

pub fn get_next_move_gremlin_tsundere(move_current: Option<usize>, other_alive_count: u8) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_PROTECT;
    }
    let last = move_current.unwrap();
    if last == IDX_MOVE_BASH {
        return IDX_MOVE_BASH;
    }
    // Last was Protect
    if other_alive_count > 0 {
        IDX_MOVE_PROTECT
    } else {
        IDX_MOVE_BASH
    }
}
