use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_attack_debuff;
use crate::monsters::make_move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Drain: Weak on the character, Strength on self
static MOVE_DRAIN: Move = make_move(
    "Drain",
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 3,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 3,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::Debuff,
);

static MOVE_POKE_5: Move = make_move_attack("Poke", 5, 2);
static MOVE_POKE_6: Move = make_move_attack("Poke", 6, 2);
static MOVE_ZAP_18: Move = make_move_attack("Zap", 18, 1);
static MOVE_ZAP_21: Move = make_move_attack("Zap", 21, 1);
static MOVE_DEBILITATE_10: Move =
    make_move_attack_debuff("Debilitate", 10, ModifierKind::Vulnerable, 2);
static MOVE_DEBILITATE_12: Move =
    make_move_attack_debuff("Debilitate", 12, ModifierKind::Vulnerable, 2);
static MOVE_HEX: Move = make_move_debuff("Hex", ModifierKind::Hex, 1, Intent::DebuffPowerful);

static MOVES_ASC0: [Move; 5] = [
    MOVE_POKE_5,
    MOVE_ZAP_18,
    MOVE_DEBILITATE_10,
    MOVE_DRAIN,
    MOVE_HEX,
];
static MOVES_ASC2: [Move; 5] = [
    MOVE_POKE_6,
    MOVE_ZAP_21,
    MOVE_DEBILITATE_12,
    MOVE_DRAIN,
    MOVE_HEX,
];

const IDX_MOVE_POKE: usize = 0;
const IDX_MOVE_ZAP: usize = 1;
const IDX_MOVE_DEBILITATE: usize = 2;
const IDX_MOVE_DRAIN: usize = 3;
const IDX_MOVE_HEX: usize = 4;

pub fn spawn_monster_chosen(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (95, 99)
    } else {
        (98, 103)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    make_entity_monster(
        MonsterName::Chosen,
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

pub fn get_next_move_chosen(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    // Openers: Hex on turn 1 at A17+; Poke then Hex below
    let turns_taken = move_history.len();
    if turns_taken == 0 {
        return if ascension_level >= 17 {
            IDX_MOVE_HEX
        } else {
            IDX_MOVE_POKE
        };
    }
    if ascension_level < 17 && turns_taken == 1 {
        return IDX_MOVE_HEX;
    }

    let last = *move_history.last().unwrap() as usize;
    if last != IDX_MOVE_DEBILITATE && last != IDX_MOVE_DRAIN {
        if rng.random_range(0..=99) < 50 {
            IDX_MOVE_DEBILITATE
        } else {
            IDX_MOVE_DRAIN
        }
    } else if rng.random_range(0..=99) < 40 {
        IDX_MOVE_ZAP
    } else {
        IDX_MOVE_POKE
    }
}
