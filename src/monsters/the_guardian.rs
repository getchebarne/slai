use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::has_modifier;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::monsters::move_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;

const MODE_SHIFT_STACKS_30: i16 = 30;
const MODE_SHIFT_STACKS_35: i16 = 35;
const MODE_SHIFT_STACKS_40: i16 = 40;
pub const DEFENSIVE_MODE_BLOCK: u16 = 20;

// Twin Slam: two hits, ModeShift refresh, SharpHide drop
const fn move_twin_slam(mode_shift_stacks: i16) -> Move {
    make_move(
        "Twin Slam",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: 8,
                    lifesteal: false,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: 8,
                    lifesteal: false,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::ModeShift,
                    stacks: mode_shift_stacks,
                },
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::ModifierRemove {
                    kind: ModifierKind::SharpHide,
                },
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::AttackBuff {
            damage: 8,
            instances: 2,
        },
    )
}

static MOVE_CHARGING_UP: Move = make_move(
    "Charging Up",
    &[Effect {
        kind: EffectKind::BlockGain { amount: 9 },
        id_source: None,
        target: TARGET_SOURCE,
    }],
    Intent::Block,
);
static MOVE_FIERCE_BASH_32: Move = move_attack("Fierce Bash", 32, 1);
static MOVE_FIERCE_BASH_36: Move = move_attack("Fierce Bash", 36, 1);
static MOVE_VENT_STEAM: Move = make_move(
    "Vent Steam",
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::DebuffPowerful,
);
static MOVE_WHIRLWIND: Move = make_move(
    "Whirlwind",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 5,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 5,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 5,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 5,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::Attack {
        damage: 5,
        instances: 4,
    },
);
static MOVE_DEFENSIVE_MODE_3: Move = move_buff("Defensive Frame", ModifierKind::SharpHide, 3);
static MOVE_DEFENSIVE_MODE_4: Move = move_buff("Defensive Frame", ModifierKind::SharpHide, 4);
static MOVE_ROLL_ATTACK_9: Move = move_attack("Roll Attack", 9, 1);
static MOVE_ROLL_ATTACK_10: Move = move_attack("Roll Attack", 10, 1);
static MOVE_TWIN_SLAM_30: Move = move_twin_slam(MODE_SHIFT_STACKS_30);
static MOVE_TWIN_SLAM_35: Move = move_twin_slam(MODE_SHIFT_STACKS_35);
static MOVE_TWIN_SLAM_40: Move = move_twin_slam(MODE_SHIFT_STACKS_40);
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

pub static THE_GUARDIAN: MonsterTemplate = MonsterTemplate {
    name: MonsterName::TheGuardian,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (240, 240)), (9, (250, 250))],
    block_start: 0,
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (4, &[&MOVES_ASC4]),
        (9, &[&MOVES_ASC9]),
        (19, &[&MOVES_ASC19]),
    ],
    modifier_tiers: &[
        (
            0,
            &[modifier_fixed(
                ModifierKind::ModeShift,
                MODE_SHIFT_STACKS_30,
            )],
        ),
        (
            9,
            &[modifier_fixed(
                ModifierKind::ModeShift,
                MODE_SHIFT_STACKS_35,
            )],
        ),
        (
            19,
            &[modifier_fixed(
                ModifierKind::ModeShift,
                MODE_SHIFT_STACKS_40,
            )],
        ),
    ],
};

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

    if has_modifier(modifiers, ModifierKind::ModeShift) {
        match move_last {
            IDX_MOVE_CHARGING_UP => IDX_MOVE_FIERCE_BASH,
            IDX_MOVE_FIERCE_BASH => IDX_MOVE_VENT_STEAM,
            IDX_MOVE_VENT_STEAM => IDX_MOVE_WHIRLWIND,
            IDX_MOVE_WHIRLWIND => IDX_MOVE_CHARGING_UP,
            IDX_MOVE_TWIN_SLAM => IDX_MOVE_WHIRLWIND,
            _ => unreachable!(
                "Invalid 'The Guardian' move in offensive frame: {}",
                move_last
            ),
        }
    } else if has_modifier(modifiers, ModifierKind::SharpHide) {
        match move_last {
            IDX_MOVE_DEFENSIVE_MODE => IDX_MOVE_ROLL_ATTACK,
            IDX_MOVE_ROLL_ATTACK => IDX_MOVE_TWIN_SLAM,
            _ => unreachable!(
                "Invalid 'The Guardian' move in defensive frame: {}",
                move_last
            ),
        }
    } else {
        IDX_MOVE_DEFENSIVE_MODE
    }
}
