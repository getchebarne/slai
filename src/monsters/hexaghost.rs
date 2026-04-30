use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS};
use crate::types::{CardName, MonsterKind, MonsterName, Vitals};

// Hexaghost (Boss). T1 Activate (Unknown), T2 Divider (HP/12+1 × 6), then a
// fixed 7-move cycle [Sear, Tackle, Sear, Inflame, Tackle, Sear, Inferno]
// repeating. Cycle position derived from move_history by walking back to the
// most recent Divider/Inferno anchor. After the first Inferno, Sear spawns
// upgraded Burns

static MOVE_ACTIVATE: Move = Move {
    name: "Activate",
    effects: &[],
    intent: Intent::Unknown,
};

// Divider's true damage (HP/12+1 × 6) is computed at fire time by
// `EffectKind::HexaghostDivider`; the placeholder Intent values shown here
// are overridden in the view layer to display the dynamic damage
static MOVE_DIVIDER: Move = Move {
    name: "Divider",
    effects: &[Effect {
        kind: EffectKind::HexaghostDivider { hits: 6 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 1,
        instances: 6,
    },
};

static MOVE_SEAR_BURN_1_NORMAL: Move = Move {
    name: "Sear",
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
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Burn,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 6,
        instances: 1,
    },
};
static MOVE_SEAR_BURN_1_UPGRADED: Move = Move {
    name: "Sear",
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
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Burn,
                count: 1,
                upgraded: true,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 6,
        instances: 1,
    },
};
static MOVE_SEAR_BURN_2_NORMAL: Move = Move {
    name: "Sear",
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
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Burn,
                count: 2,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 6,
        instances: 1,
    },
};
static MOVE_SEAR_BURN_2_UPGRADED: Move = Move {
    name: "Sear",
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
            kind: EffectKind::CardAddToDiscard {
                card_name: CardName::Burn,
                count: 2,
                upgraded: true,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 6,
        instances: 1,
    },
};

static MOVE_TACKLE_5: Move = Move {
    name: "Tackle",
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
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 5,
        instances: 2,
    },
};
static MOVE_TACKLE_6: Move = Move {
    name: "Tackle",
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
            kind: EffectKind::DamagePhysical { amount: 6 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 6,
        instances: 2,
    },
};

static MOVE_INFLAME_2: Move = Move {
    name: "Inflame",
    effects: &[
        Effect {
            kind: EffectKind::BlockGain { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::BlockBuff,
};
static MOVE_INFLAME_3: Move = Move {
    name: "Inflame",
    effects: &[
        Effect {
            kind: EffectKind::BlockGain { amount: 12 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 3,
            },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::BlockBuff,
};

// Inferno: 6 hits + BurnIncreaseAction (upgrade existing Burns + add 3 upgraded)
static MOVE_INFERNO_2: Move = Move {
    name: "Inferno",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 2 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 2 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 2 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 2 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 2 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 2 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::HexaghostBurnIncrease { count: 3 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 2,
        instances: 6,
    },
};
static MOVE_INFERNO_3: Move = Move {
    name: "Inferno",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 3 },
            id_source: None,
            target: Target::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::HexaghostBurnIncrease { count: 3 },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    intent: Intent::AttackDebuff {
        damage: 3,
        instances: 6,
    },
};

// Asc 0-3: Tackle 5, Inferno 2, Sear 1 Burn, Inflame +2 Str
static MOVES_ASC0: [Move; 7] = [
    MOVE_ACTIVATE,
    MOVE_DIVIDER,
    MOVE_SEAR_BURN_1_NORMAL,
    MOVE_SEAR_BURN_1_UPGRADED,
    MOVE_TACKLE_5,
    MOVE_INFLAME_2,
    MOVE_INFERNO_2,
];
// Asc 4-18: Tackle 6, Inferno 3 (HP shifts to 264 at Asc 9 — moves unchanged)
static MOVES_ASC4: [Move; 7] = [
    MOVE_ACTIVATE,
    MOVE_DIVIDER,
    MOVE_SEAR_BURN_1_NORMAL,
    MOVE_SEAR_BURN_1_UPGRADED,
    MOVE_TACKLE_6,
    MOVE_INFLAME_2,
    MOVE_INFERNO_3,
];
// Asc 19+: Sear 2 Burns, Inflame +3 Str
static MOVES_ASC19: [Move; 7] = [
    MOVE_ACTIVATE,
    MOVE_DIVIDER,
    MOVE_SEAR_BURN_2_NORMAL,
    MOVE_SEAR_BURN_2_UPGRADED,
    MOVE_TACKLE_6,
    MOVE_INFLAME_3,
    MOVE_INFERNO_3,
];

const IDX_MOVE_ACTIVATE: usize = 0;
pub const IDX_MOVE_DIVIDER: usize = 1;
const IDX_MOVE_SEAR_PRE: usize = 2;
const IDX_MOVE_SEAR_POST: usize = 3;
const IDX_MOVE_TACKLE: usize = 4;
const IDX_MOVE_INFLAME: usize = 5;
const IDX_MOVE_INFERNO: usize = 6;

pub fn spawn_hexaghost(ascension_level: u8) -> Entity {
    let health_max: u16 = if ascension_level < 9 { 250 } else { 264 };

    let moves: &'static [Move] = if ascension_level < 4 {
        &MOVES_ASC0
    } else if ascension_level < 19 {
        &MOVES_ASC4
    } else {
        &MOVES_ASC19
    };

    make_entity_monster(
        MonsterName::Hexaghost,
        MonsterKind::Boss,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

// Cycle slot → move idx, with Sear variant chosen by `has_inferno`
fn cycle_slot(slot: usize, has_inferno: bool) -> usize {
    let sear = if has_inferno {
        IDX_MOVE_SEAR_POST
    } else {
        IDX_MOVE_SEAR_PRE
    };
    match slot {
        0 | 2 | 5 => sear,
        1 | 4 => IDX_MOVE_TACKLE,
        3 => IDX_MOVE_INFLAME,
        6 => IDX_MOVE_INFERNO,
        _ => unreachable!("Hexaghost cycle slot out of range: {slot}"),
    }
}

pub fn get_next_move_hexaghost(move_current: Option<usize>, move_history: &[u8]) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_ACTIVATE;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    if last == IDX_MOVE_ACTIVATE {
        return IDX_MOVE_DIVIDER;
    }

    let has_inferno = move_history
        .iter()
        .any(|&m| m == IDX_MOVE_INFERNO as u8);

    if last == IDX_MOVE_DIVIDER || last == IDX_MOVE_INFERNO {
        return cycle_slot(0, has_inferno);
    }

    // Walk back from end to find the most recent Divider/Inferno anchor.
    // `slot` = number of moves since that anchor.
    let mut slot = 0usize;
    for &m in move_history.iter().rev() {
        if m == IDX_MOVE_DIVIDER as u8 || m == IDX_MOVE_INFERNO as u8 {
            break;
        }
        slot += 1;
    }
    cycle_slot(slot, has_inferno)
}
