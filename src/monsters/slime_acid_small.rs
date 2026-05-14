use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_TACKLE_3: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 3 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 3,
        instances: 1,
    },
};
static MOVE_TACKLE_4: Move = Move {
    name: "Tackle",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 4 },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 4,
        instances: 1,
    },
};
static MOVE_LICK: Move = Move {
    name: "Lick",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Weak,
            stacks: 1,
        },
        id_source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::Single,
        },
    }],
    intent: Intent::Debuff,
};
static MOVES_ASC0: [Move; 2] = [MOVE_TACKLE_3, MOVE_LICK];
static MOVES_ASC2: [Move; 2] = [MOVE_TACKLE_4, MOVE_LICK];
static MOVES_ASC17: [Move; 2] = [MOVE_TACKLE_4, MOVE_LICK];

const IDX_MOVE_TACKLE: usize = 0;
const IDX_MOVE_LICK: usize = 1;

pub fn spawn_slime_acid_small(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (8, 12)
    } else {
        (9, 13)
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
        MonsterName::SlimeAcidSmall,
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

pub fn get_next_move_slime_acid_small(
    move_current: Option<usize>,
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        if ascension_level >= 17 {
            return IDX_MOVE_LICK;
        }
        return if rng.random_bool(0.5) {
            IDX_MOVE_TACKLE
        } else {
            IDX_MOVE_LICK
        };
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    if last == IDX_MOVE_TACKLE {
        IDX_MOVE_LICK
    } else {
        IDX_MOVE_TACKLE
    }
}
