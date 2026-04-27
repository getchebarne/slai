use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::entity::{Entity, Intent, Move, make_entity_monster};
use crate::modifier::ZERO_MODIFIERS;
use crate::types::{MonsterKind, MonsterName, Vitals};
use rand::Rng;

// Java's SpikeSlime_S constructor accepts a `poisonAmount` parameter and
// applies PoisonPower if >= 1. Always 0 for the Small Slimes encounter;
// nonzero only when summoned by SpikeSlime_L's Split (Tier 4) or by certain
// card/relic effects. Wire that parameter when Tier 4 lands.

static MOVE_TACKLE_5: Move = Move {
    name: "Tackle",
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
static MOVE_TACKLE_6: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 6 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
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
