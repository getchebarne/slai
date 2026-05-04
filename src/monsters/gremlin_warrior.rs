use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::{ModifierKind, ZERO_MODIFIERS, modifier_apply};
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_SCRATCH_4: Move = Move {
    name: "Scratch",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 4,
        instances: 1,
    },
};
static MOVE_SCRATCH_5: Move = Move {
    name: "Scratch",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 5 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Attack {
        damage: 5,
        instances: 1,
    },
};
static MOVES_ASC0: [Move; 1] = [MOVE_SCRATCH_4];
static MOVES_ASC2: [Move; 1] = [MOVE_SCRATCH_5];
static MOVES_ASC17: [Move; 1] = [MOVE_SCRATCH_5];

const IDX_MOVE_SCRATCH: usize = 0;

pub fn spawn_gremlin_warrior(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (20, 24)
    } else {
        (21, 25)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let angry_stacks: i16 = if ascension_level >= 17 { 2 } else { 1 };
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Angry, angry_stacks);

    make_entity_monster(
        MonsterName::GremlinWarrior,
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

pub fn get_next_move_gremlin_warrior() -> usize {
    // Single-move monster: Scratch every turn
    IDX_MOVE_SCRATCH
}
