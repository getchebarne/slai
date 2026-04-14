use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::modifier::modifiers_new;
use crate::entities::Intent;
use crate::entities::MAX_MOVE_HISTORY;
use crate::entities::Monster;
use crate::entities::Move;
use crate::types::Vitals;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_BITE: Move = Move {
    name: "Bite",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 6 },
        source: None,
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
static MOVE_GROW_3: Move = Move {
    name: "Grow",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 3,
        },
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
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
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
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
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Buff,
};
static MOVES_ASC0: [Move; 2] = [MOVE_GROW_3, MOVE_BITE];
static MOVES_ASC2: [Move; 2] = [MOVE_GROW_4, MOVE_BITE];
static MOVES_ASC17: [Move; 2] = [MOVE_GROW_5, MOVE_BITE];

pub fn spawn_fungi_beast(ascension_level: u8, rng: &mut impl Rng) -> Monster {
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

    let vitals = Vitals {
        health: health_max,
        health_max,
        block: 0,
    };
    let mut modifiers = modifiers_new();
    modifier_apply(&mut modifiers, ModifierKind::SporeCloud, 2);

    Monster {
        name: MonsterName::FungiBeast,
        monster_kind: MonsterKind::Normal,
        vitals,
        modifiers,
        moves,
        move_current: None,
        move_history: [0; MAX_MOVE_HISTORY],
        move_history_len: 0,
        dead: false,
    }
}

pub fn get_next_move_fungi_beast(
    _move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..99);
    if roll < 60 {
        if move_history.ends_with(&[1, 1]) {
            0
        } else {
            1
        }
    } else if move_history.last().copied() == Some(0) {
        1
    } else {
        0
    }
}
