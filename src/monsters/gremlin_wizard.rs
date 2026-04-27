use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::ZERO_MODIFIERS;
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

// Gremlin Wizard. Two-charges-then-Magic state machine derived from
// `move_history.ends_with([CHARGE, CHARGE])`. No per-entity counter.
//
// Asc 0–16:  Charge, Charge, Magic, Charge, Charge, Magic, …
// Asc 17+:   Charge, Charge, Magic, Magic, Magic, … (no further charging)
//
// The Charge move has no effects — it's pure intent display (UNKNOWN). The
// state machine alone produces the right cadence; the only side effect is
// extending the move_history.

static MOVE_CHARGE: Move = Move {
    name: "Charge",
    effects: &[],
    intent: Intent::Unknown,
};
static MOVE_MAGIC_25: Move = Move {
    name: "Magic",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 25 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 25,
        instances: 1,
    },
};
static MOVE_MAGIC_30: Move = Move {
    name: "Magic",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 30 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 30,
        instances: 1,
    },
};
static MOVES_ASC0: [Move; 2] = [MOVE_CHARGE, MOVE_MAGIC_25];
static MOVES_ASC2: [Move; 2] = [MOVE_CHARGE, MOVE_MAGIC_30];
static MOVES_ASC17: [Move; 2] = [MOVE_CHARGE, MOVE_MAGIC_30];

const IDX_MOVE_CHARGE: usize = 0;
const IDX_MOVE_MAGIC: usize = 1;

pub fn spawn_gremlin_wizard(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (21, 25)
    } else {
        (22, 26)
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
        MonsterName::GremlinWizard,
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

pub fn get_next_move_gremlin_wizard(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_CHARGE;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    if last == IDX_MOVE_MAGIC {
        return if ascension_level >= 17 {
            IDX_MOVE_MAGIC
        } else {
            IDX_MOVE_CHARGE
        };
    }
    // last was Charge — fire Magic only after two Charges in a row.
    if move_history.ends_with(&[IDX_MOVE_CHARGE as u8, IDX_MOVE_CHARGE as u8]) {
        IDX_MOVE_MAGIC
    } else {
        IDX_MOVE_CHARGE
    }
}
