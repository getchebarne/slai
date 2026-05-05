use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_CHOMP_11: Move = Move {
    name: "Chomp",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 11 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 11,
        instances: 1,
    },
};
static MOVE_CHOMP_12: Move = Move {
    name: "Chomp",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 12 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 12,
        instances: 1,
    },
};
static MOVE_THRASH: Move = Move {
    name: "Thrash",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 7 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::BlockGain { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::AttackBlock {
        damage: 7,
        instances: 1,
    },
};
static MOVE_BELLOW_3_6: Move = Move {
    name: "Bellow",
    effects: &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 3,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::BlockBuff,
};
static MOVE_BELLOW_4_6: Move = Move {
    name: "Bellow",
    effects: &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 4,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::BlockGain { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::BlockBuff,
};
static MOVE_BELLOW_5_9: Move = Move {
    name: "Bellow",
    effects: &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 5,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::BlockGain { amount: 9 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::BlockBuff,
};
static MOVES_ASC0: [Move; 3] = [MOVE_CHOMP_11, MOVE_BELLOW_3_6, MOVE_THRASH];
static MOVES_ASC2: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_4_6, MOVE_THRASH];
static MOVES_ASC17: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_5_9, MOVE_THRASH];

const IDX_MOVE_CHOMP: usize = 0;
const IDX_MOVE_BELLOW: usize = 1;
const IDX_MOVE_THRASH: usize = 2;

pub fn spawn_jaw_worm(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (40, 44)
    } else {
        (42, 46)
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
        MonsterName::JawWorm,
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

pub fn get_next_move_jaw_worm(
    move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_CHOMP;
    }

    let roll = rng.random_range(0..=99);
    let move_last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    if roll < 25 {
        if move_last == IDX_MOVE_CHOMP {
            return if rng.random_bool(0.5625) {
                IDX_MOVE_BELLOW
            } else {
                IDX_MOVE_THRASH
            };
        }
        IDX_MOVE_CHOMP
    } else if roll < 55 {
        if move_history.ends_with(&[IDX_MOVE_THRASH as u8, IDX_MOVE_THRASH as u8]) {
            return if rng.random_bool(0.357) {
                IDX_MOVE_CHOMP
            } else {
                IDX_MOVE_BELLOW
            };
        }
        IDX_MOVE_THRASH
    } else {
        if move_last == IDX_MOVE_BELLOW {
            return if rng.random_bool(0.416) {
                IDX_MOVE_CHOMP
            } else {
                IDX_MOVE_THRASH
            };
        }
        IDX_MOVE_BELLOW
    }
}
