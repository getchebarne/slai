use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

// Red Slaver (Normal). T1 always Stab. After T1: gates a once-per-combat
// Entangle on a 25% roll, then biases between Stab (with Entangle used) and
// Scrape (no-three-in-a-row at Asc 0-16, no-two at Asc 17+).
//
// `usedEntangle` derived from history (does Entangle appear?).

static MOVE_STAB_13: Move = Move {
    name: "Stab",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 13 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 13,
        instances: 1,
    },
};
static MOVE_STAB_14: Move = Move {
    name: "Stab",
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
static MOVE_ENTANGLE: Move = Move {
    name: "Entangle",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Entangled,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::DebuffPowerful,
};
static MOVE_SCRAPE_8_VULN_1: Move = Move {
    name: "Scrape",
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
        damage: 8,
        instances: 1,
    },
};
static MOVE_SCRAPE_9_VULN_1: Move = Move {
    name: "Scrape",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 9 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
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
        damage: 9,
        instances: 1,
    },
};
static MOVE_SCRAPE_9_VULN_2: Move = Move {
    name: "Scrape",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 9 },
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
        damage: 9,
        instances: 1,
    },
};

static MOVES_ASC0: [Move; 3] = [MOVE_STAB_13, MOVE_ENTANGLE, MOVE_SCRAPE_8_VULN_1];
static MOVES_ASC2: [Move; 3] = [MOVE_STAB_14, MOVE_ENTANGLE, MOVE_SCRAPE_9_VULN_1];
static MOVES_ASC17: [Move; 3] = [MOVE_STAB_14, MOVE_ENTANGLE, MOVE_SCRAPE_9_VULN_2];

const IDX_MOVE_STAB: usize = 0;
const IDX_MOVE_ENTANGLE: usize = 1;
const IDX_MOVE_SCRAPE: usize = 2;

pub fn spawn_slaver_red(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (46, 50)
    } else {
        (48, 52)
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
        MonsterName::SlaverRed,
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

pub fn get_next_move_slaver_red(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_STAB;
    }
    let used_entangle = move_history
        .iter()
        .any(|&m| m == IDX_MOVE_ENTANGLE as u8);

    let roll = rng.random_range(0..=99);
    let last_two_stab =
        move_history.ends_with(&[IDX_MOVE_STAB as u8, IDX_MOVE_STAB as u8]);

    if roll >= 75 && !used_entangle {
        return IDX_MOVE_ENTANGLE;
    }
    if roll >= 55 && used_entangle && !last_two_stab {
        return IDX_MOVE_STAB;
    }
    if ascension_level >= 17 {
        if move_history.last().copied() != Some(IDX_MOVE_SCRAPE as u8) {
            IDX_MOVE_SCRAPE
        } else {
            IDX_MOVE_STAB
        }
    } else if !move_history.ends_with(&[IDX_MOVE_SCRAPE as u8, IDX_MOVE_SCRAPE as u8]) {
        IDX_MOVE_SCRAPE
    } else {
        IDX_MOVE_STAB
    }
}
