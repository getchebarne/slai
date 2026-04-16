use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, monster_entity};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_DARK_STRIKE: Move = Move {
    name: "Dark Strike",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 6 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 6,
        instances: 1,
    },
};
static MOVE_INCANTATION_3: Move = Move {
    name: "Incantation",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Ritual,
            stacks: 3,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Buff,
};
static MOVE_INCANTATION_4: Move = Move {
    name: "Incantation",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Ritual,
            stacks: 4,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Buff,
};
static MOVE_INCANTATION_5: Move = Move {
    name: "Incantation",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Ritual,
            stacks: 5,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Buff,
};
static MOVES_ASC0: [Move; 2] = [MOVE_INCANTATION_3, MOVE_DARK_STRIKE];
static MOVES_ASC2: [Move; 2] = [MOVE_INCANTATION_4, MOVE_DARK_STRIKE];
static MOVES_ASC17: [Move; 2] = [MOVE_INCANTATION_5, MOVE_DARK_STRIKE];

const IDX_MOVE_INCANTATION: usize = 0;
const IDX_MOVE_DARK_STRIKE: usize = 1;

pub fn spawn_cultist(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (48, 54)
    } else {
        (50, 56)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    monster_entity(
        MonsterName::Cultist,
        MonsterKind::Normal,
        Vitals { health: health_max, health_max, block: 0 },
        ZERO_MODIFIERS,
        moves,
    )
}

pub fn get_next_move_cultist(move_current: Option<usize>, _move_history: &[u8]) -> usize {
    if move_current.is_none() { IDX_MOVE_INCANTATION } else { IDX_MOVE_DARK_STRIKE }
}
