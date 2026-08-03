use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

static MOVE_SMASH_4_W1: Move = make_move_attack_debuff("Smash", 4, ModifierKind::Weak, 1);
static MOVE_SMASH_5_W1: Move = make_move_attack_debuff("Smash", 5, ModifierKind::Weak, 1);
static MOVE_SMASH_5_W1_F1: Move = make_move(
    "Smash",
    &[
        Effect {
            kind: EffectKind::DamagePhysical { amount: 5 },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Weak,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Frail,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
    ],
    Intent::AttackDebuff {
        damage: 5,
        instances: 1,
    },
);
static MOVES_ASC0: [Move; 1] = [MOVE_SMASH_4_W1];
static MOVES_ASC2: [Move; 1] = [MOVE_SMASH_5_W1];
static MOVES_ASC17: [Move; 1] = [MOVE_SMASH_5_W1_F1];

pub fn spawn_monster_gremlin_fat(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 7 {
        (13, 17)
    } else {
        (14, 18)
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
        MonsterName::GremlinFat,
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
