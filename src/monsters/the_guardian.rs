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
use crate::modifier::Modifiers;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::modifier::modifier_has;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;

const MODE_SHIFT_STACKS_30: i16 = 30;
const MODE_SHIFT_STACKS_35: i16 = 35;
const MODE_SHIFT_STACKS_40: i16 = 40;
pub const DEFENSIVE_MODE_BLOCK: u16 = 20;

static MOVE_CHARGING_UP: Move = Move {
    name: "Charging Up",
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 9 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Source,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Block,
};
static MOVE_FIERCE_BASH_32: Move = Move {
    name: "Fierce Bash",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 32 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 32,
        instances: 1,
    },
};
static MOVE_FIERCE_BASH_36: Move = Move {
    name: "Fierce Bash",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 36 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 36,
        instances: 1,
    },
};
static MOVE_VENT_STEAM: Move = Move {
    name: "Vent Steam",
    effects: &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::DebuffPowerful,
};
static MOVE_WHIRLWIND: Move = Move {
    name: "Whirlwind",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::Attack {
        damage: 5,
        instances: 4,
    },
};
static MOVE_DEFENSIVE_MODE_3: Move = Move {
    name: "Defensive Mode",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::SharpHide,
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
static MOVE_DEFENSIVE_MODE_4: Move = Move {
    name: "Defensive Mode",
    effects: &[Effect {
        kind: EffectKind::ModifierGain {
            kind: ModifierKind::SharpHide,
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
static MOVE_ROLL_ATTACK_9: Move = Move {
    name: "Roll Attack",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 9 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 9,
        instances: 1,
    },
};
static MOVE_ROLL_ATTACK_10: Move = Move {
    name: "Roll Attack",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { amount: 10 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    }],
    intent: Intent::Attack {
        damage: 10,
        instances: 1,
    },
};
static MOVE_TWIN_SLAM_30: Move = Move {
    name: "Twin Slam",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::ModeShift,
                stacks: MODE_SHIFT_STACKS_30,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::SharpHide,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::AttackBuff {
        damage: 8,
        instances: 2,
    },
};
static MOVE_TWIN_SLAM_35: Move = Move {
    name: "Twin Slam",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::ModeShift,
                stacks: MODE_SHIFT_STACKS_35,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::SharpHide,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::AttackBuff {
        damage: 8,
        instances: 2,
    },
};
static MOVE_TWIN_SLAM_40: Move = Move {
    name: "Twin Slam",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { amount: 8 },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::ModeShift,
                stacks: MODE_SHIFT_STACKS_40,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::SharpHide,
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Source,
                selection_kind: SelectionKind::Single,
            },
        },
    ],
    intent: Intent::AttackBuff {
        damage: 8,
        instances: 2,
    },
};
static MOVES_ASC0: [Move; 7] = [
    MOVE_CHARGING_UP,
    MOVE_FIERCE_BASH_32,
    MOVE_VENT_STEAM,
    MOVE_WHIRLWIND,
    MOVE_DEFENSIVE_MODE_3,
    MOVE_ROLL_ATTACK_9,
    MOVE_TWIN_SLAM_30,
];
static MOVES_ASC4: [Move; 7] = [
    MOVE_CHARGING_UP,
    MOVE_FIERCE_BASH_36,
    MOVE_VENT_STEAM,
    MOVE_WHIRLWIND,
    MOVE_DEFENSIVE_MODE_3,
    MOVE_ROLL_ATTACK_10,
    MOVE_TWIN_SLAM_30,
];
static MOVES_ASC9: [Move; 7] = [
    MOVE_CHARGING_UP,
    MOVE_FIERCE_BASH_36,
    MOVE_VENT_STEAM,
    MOVE_WHIRLWIND,
    MOVE_DEFENSIVE_MODE_3,
    MOVE_ROLL_ATTACK_10,
    MOVE_TWIN_SLAM_35,
];
static MOVES_ASC19: [Move; 7] = [
    MOVE_CHARGING_UP,
    MOVE_FIERCE_BASH_36,
    MOVE_VENT_STEAM,
    MOVE_WHIRLWIND,
    MOVE_DEFENSIVE_MODE_4,
    MOVE_ROLL_ATTACK_10,
    MOVE_TWIN_SLAM_40,
];

const IDX_MOVE_CHARGING_UP: usize = 0;
const IDX_MOVE_FIERCE_BASH: usize = 1;
const IDX_MOVE_VENT_STEAM: usize = 2;
const IDX_MOVE_WHIRLWIND: usize = 3;
const IDX_MOVE_DEFENSIVE_MODE: usize = 4;
const IDX_MOVE_ROLL_ATTACK: usize = 5;
pub const IDX_MOVE_TWIN_SLAM: usize = 6;

pub fn spawn_monster_the_guardian(ascension_level: u8) -> Entity {
    let health_max = if ascension_level < 9 { 240 } else { 250 };

    let moves: &'static [Move] = if ascension_level < 4 {
        &MOVES_ASC0
    } else if ascension_level < 9 {
        &MOVES_ASC4
    } else if ascension_level < 19 {
        &MOVES_ASC9
    } else {
        &MOVES_ASC19
    };

    let mode_shift_stacks = if ascension_level < 9 {
        MODE_SHIFT_STACKS_30
    } else if ascension_level < 19 {
        MODE_SHIFT_STACKS_35
    } else {
        MODE_SHIFT_STACKS_40
    };
    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::ModeShift, mode_shift_stacks);

    make_entity_monster(
        MonsterName::TheGuardian,
        MonsterKind::Boss,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        modifiers,
        moves,
    )
}

pub fn get_next_move_the_guardian_full(
    move_current: Option<usize>,
    move_history: &[u8],
    modifiers: &Modifiers,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_CHARGING_UP;
    }
    let move_last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    if modifier_has(modifiers, ModifierKind::ModeShift) {
        match move_last {
            IDX_MOVE_CHARGING_UP => IDX_MOVE_FIERCE_BASH,
            IDX_MOVE_FIERCE_BASH => IDX_MOVE_VENT_STEAM,
            IDX_MOVE_VENT_STEAM => IDX_MOVE_WHIRLWIND,
            IDX_MOVE_WHIRLWIND => IDX_MOVE_CHARGING_UP,
            IDX_MOVE_TWIN_SLAM => IDX_MOVE_WHIRLWIND,
            _ => unreachable!(
                "Invalid 'The Guardian' move in offensive mode: {}",
                move_last
            ),
        }
    } else if modifier_has(modifiers, ModifierKind::SharpHide) {
        match move_last {
            IDX_MOVE_DEFENSIVE_MODE => IDX_MOVE_ROLL_ATTACK,
            IDX_MOVE_ROLL_ATTACK => IDX_MOVE_TWIN_SLAM,
            _ => unreachable!(
                "Invalid 'The Guardian' move in defensive mode: {}",
                move_last
            ),
        }
    } else {
        IDX_MOVE_DEFENSIVE_MODE
    }
}
