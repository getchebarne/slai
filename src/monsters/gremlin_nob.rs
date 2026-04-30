use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

// Gremlin Nob (Elite). T1 always Bellow → gain Enrage 2 (Asc 18+: 3).
// Then Bull Rush / Skull Bash rotation. Java's `usedBellow` flag is derived
// from `move_history` — Bellow appears once, never again.
//
// Asc 0–17: 33% Skull Bash; otherwise Bull Rush (no-3-Bull-Rush-in-a-row →
//           force Skull Bash).
// Asc 18+:  if neither of the last 2 moves was Skull Bash → Skull Bash;
//           otherwise Bull Rush. Effectively cycles Bellow, SB, BR, BR, SB,
//           BR, BR, SB, ... (matches wiki).

static MOVE_BELLOW_2: Move = Move {
    name: "Bellow",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Enrage,
            stacks: 2,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Buff,
};
static MOVE_BELLOW_3: Move = Move {
    name: "Bellow",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Enrage,
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
static MOVE_BULL_RUSH_14: Move = Move {
    name: "Bull Rush",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 14 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 14,
        instances: 1,
    },
};
static MOVE_BULL_RUSH_16: Move = Move {
    name: "Bull Rush",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 16 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 16,
        instances: 1,
    },
};
static MOVE_SKULL_BASH_6: Move = Move {
    name: "Skull Bash",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 6,
        instances: 1,
    },
};
static MOVE_SKULL_BASH_8: Move = Move {
    name: "Skull Bash",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 8,
        instances: 1,
    },
};

static MOVES_ASC0: [Move; 3] = [MOVE_BELLOW_2, MOVE_BULL_RUSH_14, MOVE_SKULL_BASH_6];
static MOVES_ASC3: [Move; 3] = [MOVE_BELLOW_2, MOVE_BULL_RUSH_16, MOVE_SKULL_BASH_8];
static MOVES_ASC18: [Move; 3] = [MOVE_BELLOW_3, MOVE_BULL_RUSH_16, MOVE_SKULL_BASH_8];

const IDX_MOVE_BELLOW: usize = 0;
const IDX_MOVE_BULL_RUSH: usize = 1;
const IDX_MOVE_SKULL_BASH: usize = 2;

pub fn spawn_gremlin_nob(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (82, 86)
    } else {
        (85, 90)
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
        MonsterName::GremlinNob,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

pub fn get_next_move_gremlin_nob(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    // First turn: always Bellow. Derived from history (does Bellow appear?).
    let bellow_used = move_history.iter().any(|&m| m == IDX_MOVE_BELLOW as u8);
    if !bellow_used {
        return IDX_MOVE_BELLOW;
    }

    if ascension_level >= 18 {
        // Skull Bash if neither of the last two moves was Skull Bash.
        let last = move_history.last().copied();
        let last_before = if move_history.len() >= 2 {
            Some(move_history[move_history.len() - 2])
        } else {
            None
        };
        let recent_skull_bash = last == Some(IDX_MOVE_SKULL_BASH as u8)
            || last_before == Some(IDX_MOVE_SKULL_BASH as u8);
        if !recent_skull_bash {
            return IDX_MOVE_SKULL_BASH;
        }
        IDX_MOVE_BULL_RUSH
    } else {
        // Asc 0–17: 33% Skull Bash; else Bull Rush with no-3-in-a-row constraint.
        let roll = rng.random_range(0..=99);
        if roll < 33 {
            return IDX_MOVE_SKULL_BASH;
        }
        if move_history.ends_with(&[IDX_MOVE_BULL_RUSH as u8, IDX_MOVE_BULL_RUSH as u8]) {
            IDX_MOVE_SKULL_BASH
        } else {
            IDX_MOVE_BULL_RUSH
        }
    }
}
