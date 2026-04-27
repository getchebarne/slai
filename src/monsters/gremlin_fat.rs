use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

// Fat Gremlin (`GremlinFat`). Single-move monster: Smash applies physical
// damage + Weak, plus Frail at Asc 17+.
//
// Java's GremlinFat has an Escape mechanism (deathReact → switch intent to
// ESCAPE when a sibling dies). Wired in Tier 4 alongside the rest of the
// gremlin Escape network — for now, fights to death.

static MOVE_SMASH_4_W1: Move = Move {
    name: "Smash",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 4 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 4,
        instances: 1,
    },
};
static MOVE_SMASH_5_W1: Move = Move {
    name: "Smash",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 5,
        instances: 1,
    },
};
static MOVE_SMASH_5_W1_F1: Move = Move {
    name: "Smash",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Frail,
                stacks: 1,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 5,
        instances: 1,
    },
};
static MOVES_ASC0: [Move; 1] = [MOVE_SMASH_4_W1];
static MOVES_ASC2: [Move; 1] = [MOVE_SMASH_5_W1];
static MOVES_ASC17: [Move; 1] = [MOVE_SMASH_5_W1_F1];

const IDX_MOVE_SMASH: usize = 0;

pub fn spawn_gremlin_fat(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (13, 17)
    } else {
        (14, 18)
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
        MonsterName::GremlinFat,
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

pub fn get_next_move_gremlin_fat() -> usize {
    IDX_MOVE_SMASH
}
