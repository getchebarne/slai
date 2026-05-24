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
use crate::modifier::modifier_apply;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_BITE: Move = Move {
    name: "Bite",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 6 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 6,
        instances: 1,
    },
};
static MOVE_GROW_3: Move = Move {
    name: "Grow",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 3,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Source,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Buff,
};
static MOVE_GROW_4: Move = Move {
    name: "Grow",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 4,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Source,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Buff,
};
static MOVE_GROW_5: Move = Move {
    name: "Grow",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 5,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Source,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Buff,
};
static MOVES_ASC0: [Move; 2] = [MOVE_GROW_3, MOVE_BITE];
static MOVES_ASC2: [Move; 2] = [MOVE_GROW_4, MOVE_BITE];
static MOVES_ASC17: [Move; 2] = [MOVE_GROW_5, MOVE_BITE];

const IDX_MOVE_GROW: usize = 0;
const IDX_MOVE_BITE: usize = 1;

pub fn spawn_fungi_beast(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (22, 28)
    } else {
        (24, 28)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::SporeCloud, 2);

    make_entity_monster(
        MonsterName::FungiBeast,
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

pub fn get_next_move_fungi_beast(move_history: &[u8], rng: &mut impl Rng) -> usize {
    let roll = rng.random_range(0..=99);
    if roll < 60 {
        if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_GROW
        } else {
            IDX_MOVE_BITE
        }
    } else if move_history.last().copied() == Some(IDX_MOVE_GROW as u8) {
        IDX_MOVE_BITE
    } else {
        IDX_MOVE_GROW
    }
}
