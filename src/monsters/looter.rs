use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS, modifier_apply};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

// Looter (Normal). Steals gold via Mug/Lunge; runs after 4–5 turns. The
// stolen gold returns to the player on death (process_effect_death.rs hook),
// is kept on Escape (process_effect_escape_monster.rs skips the gold path).
//
// Sequence (`slash_count` derived from history as count of Mug + Lunge):
//   T1: Mug
//   T2: Mug
//   T3: 50/50 SmokeBomb (block 6) / Lunge
//   T4: SmokeBomb if T3 was Lunge; Escape if T3 was SmokeBomb
//   T5: Escape if T4 was SmokeBomb (i.e. T3 was Lunge branch)

static MOVE_MUG_10_STEAL_15: Move = Move {
    name: "Mug",
    effects: &[
        Effect {
            kind: EffectKind::GoldSteal { amount: 15 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 10 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 10,
        instances: 1,
    },
};
static MOVE_MUG_11_STEAL_15: Move = Move {
    name: "Mug",
    effects: &[
        Effect {
            kind: EffectKind::GoldSteal { amount: 15 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 11 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 11,
        instances: 1,
    },
};
static MOVE_MUG_11_STEAL_20: Move = Move {
    name: "Mug",
    effects: &[
        Effect {
            kind: EffectKind::GoldSteal { amount: 20 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 11 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 11,
        instances: 1,
    },
};
static MOVE_LUNGE_12_STEAL_15: Move = Move {
    name: "Lunge",
    effects: &[
        Effect {
            kind: EffectKind::GoldSteal { amount: 15 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 12,
        instances: 1,
    },
};
static MOVE_LUNGE_14_STEAL_15: Move = Move {
    name: "Lunge",
    effects: &[
        Effect {
            kind: EffectKind::GoldSteal { amount: 15 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 14 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 14,
        instances: 1,
    },
};
static MOVE_LUNGE_14_STEAL_20: Move = Move {
    name: "Lunge",
    effects: &[
        Effect {
            kind: EffectKind::GoldSteal { amount: 20 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 14 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 14,
        instances: 1,
    },
};
static MOVE_SMOKE_BOMB: Move = Move {
    name: "Smoke Bomb",
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 6 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Block,
};
static MOVE_ESCAPE: Move = Move {
    name: "Escape",
    effects: &[Effect {
        kind: EffectKind::EscapeMonster,
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Escape,
};

static MOVES_ASC0: [Move; 4] = [
    MOVE_MUG_10_STEAL_15,
    MOVE_LUNGE_12_STEAL_15,
    MOVE_SMOKE_BOMB,
    MOVE_ESCAPE,
];
static MOVES_ASC2: [Move; 4] = [
    MOVE_MUG_11_STEAL_15,
    MOVE_LUNGE_14_STEAL_15,
    MOVE_SMOKE_BOMB,
    MOVE_ESCAPE,
];
static MOVES_ASC17: [Move; 4] = [
    MOVE_MUG_11_STEAL_20,
    MOVE_LUNGE_14_STEAL_20,
    MOVE_SMOKE_BOMB,
    MOVE_ESCAPE,
];

const IDX_MOVE_MUG: usize = 0;
const IDX_MOVE_LUNGE: usize = 1;
const IDX_MOVE_SMOKE_BOMB: usize = 2;
const IDX_MOVE_ESCAPE: usize = 3;

pub fn spawn_looter(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (44, 48)
    } else {
        (46, 50)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let gold_amt: i16 = if ascension_level < 17 { 15 } else { 20 };
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Thievery, gold_amt);

    make_entity_monster(
        MonsterName::Looter,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}

pub fn get_next_move_looter(
    move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_MUG;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    match last {
        IDX_MOVE_MUG => {
            // Count of Mug + Lunge in history (the "slash_count").
            let slash_count = move_history
                .iter()
                .filter(|&&m| m == IDX_MOVE_MUG as u8 || m == IDX_MOVE_LUNGE as u8)
                .count();
            if slash_count < 2 {
                IDX_MOVE_MUG
            } else if rng.random_bool(0.5) {
                IDX_MOVE_SMOKE_BOMB
            } else {
                IDX_MOVE_LUNGE
            }
        }
        IDX_MOVE_LUNGE => IDX_MOVE_SMOKE_BOMB,
        IDX_MOVE_SMOKE_BOMB => IDX_MOVE_ESCAPE,
        IDX_MOVE_ESCAPE => IDX_MOVE_ESCAPE,
        _ => unreachable!("Looter unexpected move idx: {last}"),
    }
}
