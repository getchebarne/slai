use crate::consts::HEXAGHOST_DIVIDER_HITS;
use crate::consts::MAX_EFFECTS_PER_MOVE;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::effect::ZERO_EFFECT;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack_card_add;
use crate::types::CardName;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;

// First move: essentially a no-op
const INFERNO_HITS: usize = 6;

// Inferno: six hits then the burn upgrade; fills all MAX_EFFECTS_PER_MOVE slots
const fn make_move_inferno(damage: u16) -> Move {
    let mut effects = [ZERO_EFFECT; MAX_EFFECTS_PER_MOVE];
    let mut i = 0;
    while i < INFERNO_HITS {
        effects[i] = Effect {
            kind: EffectKind::DamagePhysical { amount: damage },
            id_source: None,
            target: TARGET_CHARACTER,
        };
        i += 1;
    }
    effects[INFERNO_HITS] = Effect {
        kind: EffectKind::HexaghostBurnIncrease { count: 3 },
        id_source: None,
        target: Target::Direct(None),
    };
    Move {
        name: "Inferno",
        effects,
        effects_len: (INFERNO_HITS + 1) as u8,
        intent: Intent::AttackDebuff {
            damage,
            instances: INFERNO_HITS as u8,
        },
    }
}

static MOVE_ACTIVATE: Move = make_move("Activate", &[], Intent::Unknown);

// Divider true damage (HP/12+1 x 6); amounts and intent locked in at move selection
static DIVIDER_HIT: Effect = Effect {
    kind: EffectKind::DamagePhysical { amount: 0 },
    id_source: None,
    target: TARGET_CHARACTER,
};
static MOVE_DIVIDER: Move = make_move(
    "Divider",
    &[DIVIDER_HIT; HEXAGHOST_DIVIDER_HITS as usize],
    Intent::Attack {
        damage: 0, // Placeholder
        instances: HEXAGHOST_DIVIDER_HITS,
    },
);

static MOVE_SEAR_BURN_1_NORMAL: Move =
    make_move_attack_card_add("Sear", 6, CardName::Burn, 1, false);
static MOVE_SEAR_BURN_1_UPGRADED: Move =
    make_move_attack_card_add("Sear", 6, CardName::Burn, 1, true);
static MOVE_SEAR_BURN_2_NORMAL: Move =
    make_move_attack_card_add("Sear", 6, CardName::Burn, 2, false);
static MOVE_SEAR_BURN_2_UPGRADED: Move =
    make_move_attack_card_add("Sear", 6, CardName::Burn, 2, true);

static MOVE_TACKLE_5: Move = make_move(
    "Tackle",
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::Attack {
        damage: 5,
        instances: 2,
    },
);
static MOVE_TACKLE_6: Move = make_move(
    "Tackle",
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 6 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 6 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::Attack {
        damage: 6,
        instances: 2,
    },
);

static MOVE_INFLAME_2: Move = make_move(
    "Inflame",
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 12 },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::BlockBuff,
);
static MOVE_INFLAME_3: Move = make_move(
    "Inflame",
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 12 },
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 3,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::BlockBuff,
);

// Inferno: 6 hits + Burn increase (upgrade existing Burns + add 3 upgraded)
static MOVE_INFERNO_2: Move = make_move_inferno(2);
static MOVE_INFERNO_3: Move = make_move_inferno(3);

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

pub fn spawn_monster_hexaghost(ascension_level: u8) -> Entity {
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

// Cycle slot -> move idx, with Sear variant chosen by `has_inferno`
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

    // Calculate if Inferno has already occured
    let has_inferno = move_history.iter().any(|&m| m == IDX_MOVE_INFERNO as u8);

    // Cycle start
    if last == IDX_MOVE_DIVIDER || last == IDX_MOVE_INFERNO {
        return cycle_slot(0, has_inferno);
    }

    // Mid-cycle: walk back to most recent Divider/Inferno anchor; `slot` = moves since
    let mut slot = 0usize;
    for &move_ in move_history.iter().rev() {
        if move_ == IDX_MOVE_DIVIDER as u8 || move_ == IDX_MOVE_INFERNO as u8 {
            break;
        }
        slot += 1;
    }
    cycle_slot(slot, has_inferno)
}
