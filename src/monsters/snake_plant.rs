use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::MonsterKind;
use crate::types::MonsterName;
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

static MOVE_CHOMP_7: Move = move_attack("Chomp", 7, 3);
static MOVE_CHOMP_8: Move = move_attack("Chomp", 8, 3);

static MOVES_ASC0: [Move; 2] = [MOVE_CHOMP_7, MOVE_SPORES];
static MOVES_ASC2: [Move; 2] = [MOVE_CHOMP_8, MOVE_SPORES];

const IDX_MOVE_CHOMP: usize = 0;
const IDX_MOVE_SPORES: usize = 1;

pub static SNAKE_PLANT: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SnakePlant,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (75, 79)), (7, (78, 82))],
    block_start: 0,
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[(
        0,
        &[modifier_fixed(ModifierKind::Malleable, MALLEABLE_BASE)],
    )],
};

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
