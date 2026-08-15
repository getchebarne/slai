use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::MODIFIERS_ZERO;
use crate::modifier::ModifierKind;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// Lunge: the hit plus 9 self block
const fn make_move_lunge(damage: u16) -> Move {
    make_move(
        "Lunge",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    lifesteal: false,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::BlockGain { amount: 9 },
                id_source: None,
                target: TARGET_SOURCE,
            },
        ],
        Intent::AttackBlock {
            damage,
            instances: 1,
        },
    )
}

static MOVE_BEAR_HUG_2: Move =
    make_move_debuff("Bear Hug", ModifierKind::Dexterity, -2, Intent::Debuff);
static MOVE_BEAR_HUG_4: Move =
    make_move_debuff("Bear Hug", ModifierKind::Dexterity, -4, Intent::Debuff);
static MOVE_LUNGE_9: Move = make_move_lunge(9);
static MOVE_LUNGE_10: Move = make_move_lunge(10);
static MOVE_MAUL_18: Move = make_move_attack("Maul", 18, 1);
static MOVE_MAUL_20: Move = make_move_attack("Maul", 20, 1);

static MOVES_ASC0: [Move; 3] = [MOVE_BEAR_HUG_2, MOVE_LUNGE_9, MOVE_MAUL_18];
static MOVES_ASC2: [Move; 3] = [MOVE_BEAR_HUG_2, MOVE_LUNGE_10, MOVE_MAUL_20];
static MOVES_ASC17: [Move; 3] = [MOVE_BEAR_HUG_4, MOVE_LUNGE_10, MOVE_MAUL_20];

const IDX_MOVE_BEAR_HUG: usize = 0;
const IDX_MOVE_LUNGE: usize = 1;
const IDX_MOVE_MAUL: usize = 2;

pub fn spawn_monster_bandit_bear(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (38, 42)
    } else {
        (40, 44)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    make_entity_monster(
        MonsterName::BanditBear,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        MODIFIERS_ZERO,
        moves,
    )
}

// Fixed loop: Bear Hug once, then Lunge/Maul forever
pub fn get_next_move_bandit_bear(move_current: Option<usize>, move_history: &[u8]) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_BEAR_HUG;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;
    match last {
        IDX_MOVE_BEAR_HUG | IDX_MOVE_MAUL => IDX_MOVE_LUNGE,
        IDX_MOVE_LUNGE => IDX_MOVE_MAUL,
        _ => unreachable!("Bandit Bear unexpected move idx: {last}"),
    }
}
