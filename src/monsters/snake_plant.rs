use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

pub const MALLEABLE_BASE: i16 = 3;

// Enfeebling Spores: Frail and Weak together
static MOVE_SPORES: Move = make_move(
    "Enfeebling Spores",
    &[
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Frail,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 2,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::DebuffPowerful,
);

static MOVE_CHOMP_7: Move = make_move_attack("Chomp", 7, 3);
static MOVE_CHOMP_8: Move = make_move_attack("Chomp", 8, 3);

static MOVES_ASC0: [Move; 2] = [MOVE_CHOMP_7, MOVE_SPORES];
static MOVES_ASC2: [Move; 2] = [MOVE_CHOMP_8, MOVE_SPORES];

const IDX_MOVE_CHOMP: usize = 0;
const IDX_MOVE_SPORES: usize = 1;

pub fn spawn_monster_snake_plant(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (75, 79)
    } else {
        (78, 82)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else {
        &MOVES_ASC2
    };

    let mut modifiers = MODIFIERS_ZERO;
    modifier_apply(&mut modifiers, ModifierKind::Malleable, MALLEABLE_BASE);

    make_entity_monster(
        MonsterName::SnakePlant,
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

pub fn get_next_move_snake_plant(
    move_history: &[u8],
    ascension_level: u8,
    rng: &mut impl Rng,
) -> usize {
    let roll = rng.random_range(0..=99);
    if roll < 65 {
        if move_history.ends_with(&[IDX_MOVE_CHOMP as u8, IDX_MOVE_CHOMP as u8]) {
            IDX_MOVE_SPORES
        } else {
            IDX_MOVE_CHOMP
        }
    } else {
        // A17+ spaces Spores further apart: neither of the last two may be Spores
        let lookback = if ascension_level >= 17 { 2 } else { 1 };
        let spores_blocked = move_history
            .iter()
            .rev()
            .take(lookback)
            .any(|&idx_move| idx_move == IDX_MOVE_SPORES as u8);
        if spores_blocked {
            IDX_MOVE_CHOMP
        } else {
            IDX_MOVE_SPORES
        }
    }
}
