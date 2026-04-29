use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_STAB_12: Move = Move {
    name: "Stab",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 12 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 12,
        instances: 1,
    },
};
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
static MOVE_RAKE_7_W1: Move = Move {
    name: "Rake",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 7 },
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
        damage: 7,
        instances: 1,
    },
};
static MOVE_RAKE_8_W1: Move = Move {
    name: "Rake",
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
        damage: 8,
        instances: 1,
    },
};
static MOVE_RAKE_8_W2: Move = Move {
    name: "Rake",
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
                kind: ModifierKind::Weak,
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
static MOVES_ASC0: [Move; 2] = [MOVE_STAB_12, MOVE_RAKE_7_W1];
static MOVES_ASC2: [Move; 2] = [MOVE_STAB_13, MOVE_RAKE_8_W1];
static MOVES_ASC17: [Move; 2] = [MOVE_STAB_13, MOVE_RAKE_8_W2];

const IDX_MOVE_STAB: usize = 0;
const IDX_MOVE_RAKE: usize = 1;

pub fn spawn_slaver_blue(ascension_level: u8, rng: &mut impl Rng) -> Entity {
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
        MonsterName::SlaverBlue,
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

pub fn get_next_move_slaver_blue(
    _move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    let stab_twice = move_history.ends_with(&[IDX_MOVE_STAB as u8, IDX_MOVE_STAB as u8]);
    if roll >= 40 && !stab_twice {
        return IDX_MOVE_STAB;
    }
    if ascension_level >= 17 {
        let rake_last = move_history.last().copied() == Some(IDX_MOVE_RAKE as u8);
        if !rake_last {
            IDX_MOVE_RAKE
        } else {
            IDX_MOVE_STAB
        }
    } else {
        let rake_twice = move_history.ends_with(&[IDX_MOVE_RAKE as u8, IDX_MOVE_RAKE as u8]);
        if !rake_twice {
            IDX_MOVE_RAKE
        } else {
            IDX_MOVE_STAB
        }
    }
}
