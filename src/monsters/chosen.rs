use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::monsters::move_attack_debuff;
use crate::monsters::move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Drain: Weak on the Character, Strength on self
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

static MOVE_POKE_5: Move = move_attack("Poke", 5, 2);
static MOVE_POKE_6: Move = move_attack("Poke", 6, 2);
static MOVE_ZAP_18: Move = move_attack("Zap", 18, 1);
static MOVE_ZAP_21: Move = move_attack("Zap", 21, 1);
static MOVE_DEBILITATE_10: Move = move_attack_debuff("Debilitate", 10, ModifierKind::Vulnerable, 2);
static MOVE_DEBILITATE_12: Move = move_attack_debuff("Debilitate", 12, ModifierKind::Vulnerable, 2);
static MOVE_HEX: Move = move_debuff("Hex", ModifierKind::Hex, 1, Intent::DebuffPowerful);

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

pub static CHOSEN: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Chosen,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (95, 99)), (7, (98, 103))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2)],
    modifier_tiers: &[],
};

pub fn get_next_move_chosen(move_history: &[u8], ascension_level: u8, rng: &mut impl Rng) -> usize {
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
