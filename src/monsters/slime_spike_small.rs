use crate::effect::{CandidatePool, DamageCondition, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::ZERO_MODIFIERS;
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

static MOVE_TACKLE_5: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 5,
            condition: DamageCondition::Always,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 5,
        instances: 1,
    },
};
static MOVE_TACKLE_6: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical {
            amount: 6,
            condition: DamageCondition::Always,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 6,
        instances: 1,
    },
};
static MOVES_ASC0: [Move; 1] = [MOVE_TACKLE_5];
static MOVES_ASC2: [Move; 1] = [MOVE_TACKLE_6];
static MOVES_ASC17: [Move; 1] = [MOVE_TACKLE_6];

const IDX_MOVE_TACKLE: usize = 0;

pub fn spawn_slime_spike_small(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (10, 14)
    } else {
        (11, 15)
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
        MonsterName::SlimeSpikeSmall,
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

pub fn get_next_move_slime_spike_small() -> usize {
    IDX_MOVE_TACKLE
}
