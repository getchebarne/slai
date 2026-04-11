use crate::effect::CandidatePool;
use crate::effect::EffectKind;
use crate::effect::Effect;
use crate::effect::SelectionKind;
use crate::effect::Targeting;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::modifier_apply;
use crate::modifier::modifier_has;
use crate::modifier::modifiers_new;
use crate::monsters::Intent;
use crate::monsters::MAX_MOVE_HISTORY;
use crate::monsters::Monster;
use crate::monsters::Move;
use crate::state::Vitals;
use crate::types::MonsterKind;
use crate::types::MonsterName;

const MODE_SHIFT_STACKS_30: i16 = 30;
const MODE_SHIFT_STACKS_35: i16 = 35;
const MODE_SHIFT_STACKS_40: i16 = 40;

static MOVE_CHARGING_UP: Move = Move {
    name: "Charging Up",
    effects: &[Effect {
        kind: EffectKind::BlockGain { amount: 9 },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Block,
};
static MOVE_FIERCE_BASH_32: Move = Move {
    name: "Fierce Bash",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 32 },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
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
        kind: EffectKind::DamagePhysical { base: 36 },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
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
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
    ],
    intent: Intent::DebuffPowerful,
};
static MOVE_WHIRLWIND: Move = Move {
    name: "Whirlwind",
    effects: &[
        Effect {
            kind: EffectKind::DamagePhysical { base: 5 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 5 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 5 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 5 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
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
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
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
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Source,
            selection: SelectionKind::All,
        },
    }],
    intent: Intent::Buff,
};
static MOVE_ROLL_ATTACK_9: Move = Move {
    name: "Roll Attack",
    effects: &[Effect {
        kind: EffectKind::DamagePhysical { base: 9 },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
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
        kind: EffectKind::DamagePhysical { base: 10 },
        source: None,
        targeting: Targeting::Resolve {
            candidates: CandidatePool::Character,
            selection: SelectionKind::All,
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
            kind: EffectKind::DamagePhysical { base: 8 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 8 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::ModeShift,
                stacks: MODE_SHIFT_STACKS_30,
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::SharpHide,
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
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
            kind: EffectKind::DamagePhysical { base: 8 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 8 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::ModeShift,
                stacks: MODE_SHIFT_STACKS_35,
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::SharpHide,
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
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
            kind: EffectKind::DamagePhysical { base: 8 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::DamagePhysical { base: 8 },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Character,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::ModeShift,
                stacks: MODE_SHIFT_STACKS_40,
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
            },
        },
        Effect {
            kind: EffectKind::ModifierRemove {
                kind: ModifierKind::SharpHide,
            },
            source: None,
            targeting: Targeting::Resolve {
                candidates: CandidatePool::Source,
                selection: SelectionKind::All,
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
    MOVE_TWIN_SLAM_35,
];
static MOVES_ASC4: [Move; 7] = [
    MOVE_CHARGING_UP,
    MOVE_FIERCE_BASH_36,
    MOVE_VENT_STEAM,
    MOVE_WHIRLWIND,
    MOVE_DEFENSIVE_MODE_3,
    MOVE_ROLL_ATTACK_10,
    MOVE_TWIN_SLAM_35,
];
static MOVES_ASC9: [Move; 7] = [
    MOVE_CHARGING_UP,
    MOVE_FIERCE_BASH_36,
    MOVE_VENT_STEAM,
    MOVE_WHIRLWIND,
    MOVE_DEFENSIVE_MODE_3,
    MOVE_ROLL_ATTACK_10,
    MOVE_TWIN_SLAM_40,
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

pub fn spawn_the_guardian(ascension_level: u8) -> Monster {
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

    let vitals = Vitals {
        health: health_max,
        health_max,
        block: 0,
    };
    let mode_shift_stacks = if ascension_level < 9 {
        MODE_SHIFT_STACKS_30
    } else if ascension_level < 19 {
        MODE_SHIFT_STACKS_35
    } else {
        MODE_SHIFT_STACKS_40
    };
    let mut modifiers = modifiers_new();
    modifier_apply(&mut modifiers, ModifierKind::ModeShift, mode_shift_stacks);

    Monster {
        name: MonsterName::TheGuardian,
        monster_kind: MonsterKind::Boss,
        vitals,
        modifiers,
        moves,
        move_current: None,
        move_history: [0; MAX_MOVE_HISTORY],
        move_history_len: 0,
        dead: false,
    }
}

pub fn get_next_move_the_guardian_full(
    move_current: Option<usize>,
    move_history: &[u8],
    modifiers: &Modifiers,
) -> usize {
    if move_current.is_none() {
        return 0;
    }
    let move_last = move_history
        .last()
        .copied()
        .expect("`move_history` cannot be empty here");

    if modifier_has(modifiers, ModifierKind::ModeShift) {
        match move_last {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 0,
            6 => 3,
            _ => unreachable!(
                "Invalid 'The Guardian' move in offensive mode: {}",
                move_last
            ),
        }
    } else if modifier_has(modifiers, ModifierKind::SharpHide) {
        match move_last {
            4 => 5,
            5 => 6,
            _ => unreachable!(
                "Invalid 'The Guardian' move in defensive mode: {}",
                move_last
            ),
        }
    } else {
        4
    }
}
