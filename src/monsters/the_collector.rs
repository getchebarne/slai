use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

pub const TORCH_HEAD_COUNT: usize = 2;

// Spawn and Revive share one effect: top the roster back up to two Torch Heads
const fn make_move_torch_head_spawn(name: &'static str) -> Move {
    make_move(
        name,
        &[Effect {
            kind: EffectKind::TorchHeadSpawn,
            id_source: None,
            target: Target::Direct(None),
        }],
        Intent::Unknown,
    )
}

static MOVE_SPAWN: Move = make_move_torch_head_spawn("Spawn");
static MOVE_REVIVE: Move = make_move_torch_head_spawn("Revive");

// Buff: block for the Collector, Strength for the whole side
const fn make_move_buff_side(block: u16, strength: i16) -> Move {
    make_move(
        "Buff",
        &[
            Effect {
                kind: EffectKind::BlockGain { amount: block },
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: strength,
                },
                id_source: None,
                target: TARGET_MONSTERS_ALL,
            },
        ],
        Intent::BlockBuff,
    )
}

// Mega Debuff: Weak, Vulnerable and Frail all at once
const fn make_move_mega_debuff(stacks: i16) -> Move {
    make_move(
        "Mega Debuff",
        &[
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Weak,
                    stacks,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Vulnerable,
                    stacks,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Frail,
                    stacks,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::DebuffPowerful,
    )
}

static MOVE_FIREBALL_18: Move = make_move_attack("Fireball", 18, 1);
static MOVE_FIREBALL_21: Move = make_move_attack("Fireball", 21, 1);

static MOVES_ASC0: [Move; 5] = [
    MOVE_SPAWN,
    MOVE_FIREBALL_18,
    make_move_buff_side(15, 3),
    make_move_mega_debuff(3),
    MOVE_REVIVE,
];
static MOVES_ASC4: [Move; 5] = [
    MOVE_SPAWN,
    MOVE_FIREBALL_21,
    make_move_buff_side(15, 4),
    make_move_mega_debuff(3),
    MOVE_REVIVE,
];
static MOVES_ASC9: [Move; 5] = [
    MOVE_SPAWN,
    MOVE_FIREBALL_21,
    make_move_buff_side(18, 4),
    make_move_mega_debuff(3),
    MOVE_REVIVE,
];
static MOVES_ASC19: [Move; 5] = [
    MOVE_SPAWN,
    MOVE_FIREBALL_21,
    make_move_buff_side(23, 5),
    make_move_mega_debuff(5),
    MOVE_REVIVE,
];

const IDX_MOVE_SPAWN: usize = 0;
const IDX_MOVE_FIREBALL: usize = 1;
const IDX_MOVE_BUFF: usize = 2;
const IDX_MOVE_MEGA_DEBUFF: usize = 3;
const IDX_MOVE_REVIVE: usize = 4;

pub fn spawn_monster_the_collector(ascension_level: u8) -> Entity {
    let health_max = if ascension_level < 9 { 282 } else { 300 };

    let moves: &'static [Move] = if ascension_level < 4 {
        &MOVES_ASC0
    } else if ascension_level < 9 {
        &MOVES_ASC4
    } else if ascension_level < 19 {
        &MOVES_ASC9
    } else {
        &MOVES_ASC19
    };

    make_entity_monster(
        MonsterName::TheCollector,
        MonsterKind::Boss,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

pub fn get_next_move_the_collector(
    move_history: &[u8],
    torch_heads_alive: usize,
    rng: &mut impl Rng,
) -> usize {
    if move_history.is_empty() {
        return IDX_MOVE_SPAWN;
    }

    // The ultimate lands once, guaranteed on the fourth turn
    let ult_used = move_history
        .iter()
        .any(|&m| m as usize == IDX_MOVE_MEGA_DEBUFF);
    if move_history.len() >= 3 && !ult_used {
        return IDX_MOVE_MEGA_DEBUFF;
    }

    let last = *move_history.last().unwrap() as usize;
    let roll = rng.random_range(0..=99);
    if roll <= 25 && torch_heads_alive < TORCH_HEAD_COUNT && last != IDX_MOVE_REVIVE {
        IDX_MOVE_REVIVE
    } else if roll <= 70
        && !move_history.ends_with(&[IDX_MOVE_FIREBALL as u8, IDX_MOVE_FIREBALL as u8])
    {
        IDX_MOVE_FIREBALL
    } else if last == IDX_MOVE_BUFF {
        IDX_MOVE_FIREBALL
    } else {
        IDX_MOVE_BUFF
    }
}
