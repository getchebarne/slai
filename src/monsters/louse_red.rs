use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::entity::make_entity_monster;
use crate::entity::make_move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_BITE_5: Move = make_move(
    "Bite",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 5 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 5,
        instances: 1,
    },
);
static MOVE_BITE_6: Move = make_move(
    "Bite",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 6 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 6,
        instances: 1,
    },
);
static MOVE_BITE_7: Move = make_move(
    "Bite",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 7 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 7,
        instances: 1,
    },
);
static MOVE_BITE_8: Move = make_move(
    "Bite",
    &[Effect {
        kind: EffectKind::DamagePhysical { amount: 8 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Attack {
        damage: 8,
        instances: 1,
    },
);
static MOVE_STRENGTHEN_3: Move = make_move(
    "Grow",
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 3,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Source,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Buff,
);
static MOVE_STRENGTHEN_4: Move = make_move(
    "Grow",
    &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::Strength,
            stacks: 4,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Source,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    }],
    Intent::Buff,
);

// 9 move tables: 3 asc brackets × 3 bite values (5/6/7 at Asc 0-1, 6/7/8 at Asc 2+)
static MOVES_ASC0_BITE5: [Move; 2] = [MOVE_BITE_5, MOVE_STRENGTHEN_3];
static MOVES_ASC0_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_3];
static MOVES_ASC0_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_3];
static MOVES_ASC2_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_3];
static MOVES_ASC2_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_3];
static MOVES_ASC2_BITE8: [Move; 2] = [MOVE_BITE_8, MOVE_STRENGTHEN_3];
static MOVES_ASC17_BITE6: [Move; 2] = [MOVE_BITE_6, MOVE_STRENGTHEN_4];
static MOVES_ASC17_BITE7: [Move; 2] = [MOVE_BITE_7, MOVE_STRENGTHEN_4];
static MOVES_ASC17_BITE8: [Move; 2] = [MOVE_BITE_8, MOVE_STRENGTHEN_4];

const IDX_MOVE_BITE: usize = 0;
const IDX_MOVE_STRENGTHEN: usize = 1;

pub fn spawn_monster_louse_red(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (10, 15)
    } else {
        (11, 16)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let bite_dmg: u8 = if ascension_level < 2 {
        rng.random_range(5..=7)
    } else {
        rng.random_range(6..=8)
    };

    let moves: &'static [Move] = if ascension_level < 2 {
        match bite_dmg {
            5 => &MOVES_ASC0_BITE5,
            6 => &MOVES_ASC0_BITE6,
            7 => &MOVES_ASC0_BITE7,
            _ => unreachable!("Asc 0–1 bite damage must be in 5..=7"),
        }
    } else if ascension_level < 17 {
        match bite_dmg {
            6 => &MOVES_ASC2_BITE6,
            7 => &MOVES_ASC2_BITE7,
            8 => &MOVES_ASC2_BITE8,
            _ => unreachable!("Asc 2–16 bite damage must be in 6..=8"),
        }
    } else {
        match bite_dmg {
            6 => &MOVES_ASC17_BITE6,
            7 => &MOVES_ASC17_BITE7,
            8 => &MOVES_ASC17_BITE8,
            _ => unreachable!("Asc 17+ bite damage must be in 6..=8"),
        }
    };

    let curl_up_stacks: i16 = if ascension_level < 7 {
        rng.random_range(3..=7)
    } else if ascension_level < 17 {
        rng.random_range(4..=8)
    } else {
        rng.random_range(9..=12)
    };
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::CurlUp, curl_up_stacks);

    make_entity_monster(
        MonsterName::LouseNormal,
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

pub fn get_next_move_louse_red(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if ascension_level >= 17 {
        // Asc 17+: Strengthen never twice in a row; Bite no constraint
        if roll < 25 {
            if move_history.last().copied() == Some(IDX_MOVE_STRENGTHEN as u8) {
                IDX_MOVE_BITE
            } else {
                IDX_MOVE_STRENGTHEN
            }
        } else if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_STRENGTHEN
        } else {
            IDX_MOVE_BITE
        }
    } else {
        // Asc 0–16: Strengthen no-two-in-a-row; Bite no-three-in-a-row
        if roll < 25 {
            if move_history.ends_with(&[IDX_MOVE_STRENGTHEN as u8, IDX_MOVE_STRENGTHEN as u8]) {
                IDX_MOVE_BITE
            } else {
                IDX_MOVE_STRENGTHEN
            }
        } else if move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8]) {
            IDX_MOVE_STRENGTHEN
        } else {
            IDX_MOVE_BITE
        }
    }
}
