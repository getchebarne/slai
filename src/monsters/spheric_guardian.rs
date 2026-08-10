use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::modifier::modifier_apply;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack;
use crate::monsters::make_move_attack_debuff;
use crate::monsters::make_move_block;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

const START_BLOCK: u16 = 40;

// Harden: block first, then the hit
const fn make_move_harden(damage: u16) -> Move {
    make_move(
        "Harden",
        &[
            Effect {
                kind: EffectKind::BlockGain { amount: 15 },
                id_source: None,
                target: TARGET_SOURCE,
            },
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    lifesteal: false,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
        ],
        Intent::AttackBlock {
            damage,
            instances: 1,
        },
    )
}

static MOVE_SLAM_10: Move = make_move_attack("Slam", 10, 2);
static MOVE_SLAM_11: Move = make_move_attack("Slam", 11, 2);
static MOVE_ACTIVATE_25: Move = make_move_block("Activate", 25);
static MOVE_ACTIVATE_35: Move = make_move_block("Activate", 35);
static MOVE_HARDEN_10: Move = make_move_harden(10);
static MOVE_HARDEN_11: Move = make_move_harden(11);
static MOVE_FRAIL_ATTACK_10: Move =
    make_move_attack_debuff("Attack/Debuff", 10, ModifierKind::Frail, 5);
static MOVE_FRAIL_ATTACK_11: Move =
    make_move_attack_debuff("Attack/Debuff", 11, ModifierKind::Frail, 5);

static MOVES_ASC0: [Move; 4] = [
    MOVE_SLAM_10,
    MOVE_ACTIVATE_25,
    MOVE_HARDEN_10,
    MOVE_FRAIL_ATTACK_10,
];
static MOVES_ASC2: [Move; 4] = [
    MOVE_SLAM_11,
    MOVE_ACTIVATE_25,
    MOVE_HARDEN_11,
    MOVE_FRAIL_ATTACK_11,
];
static MOVES_ASC17: [Move; 4] = [
    MOVE_SLAM_11,
    MOVE_ACTIVATE_35,
    MOVE_HARDEN_11,
    MOVE_FRAIL_ATTACK_11,
];

const IDX_MOVE_SLAM: usize = 0;
const IDX_MOVE_ACTIVATE: usize = 1;
const IDX_MOVE_HARDEN: usize = 2;
const IDX_MOVE_FRAIL_ATTACK: usize = 3;

pub fn spawn_monster_spheric_guardian(ascension_level: u8, _rng: &mut impl Rng) -> Entity {
    // Flat 20 HP at every ascension; the shell is the health bar
    let health_max = 20;

    let moves: &'static [Move] = if ascension_level < 2 {
        &MOVES_ASC0
    } else if ascension_level < 17 {
        &MOVES_ASC2
    } else {
        &MOVES_ASC17
    };

    let mut modifiers = ZERO_MODIFIERS;
    modifier_apply(&mut modifiers, ModifierKind::Barricade, 1);
    modifier_apply(&mut modifiers, ModifierKind::Artifact, 3);

    make_entity_monster(
        MonsterName::SphericGuardian,
        MonsterKind::Normal,
        Vitals {
            health: health_max,
            health_max,
            block: START_BLOCK,
        },
        modifiers,
        moves,
    )
}

// Fully deterministic: Activate, Frail Attack, then Slam/Harden alternating
pub fn get_next_move_spheric_guardian(move_history: &[u8]) -> usize {
    match move_history.len() {
        0 => IDX_MOVE_ACTIVATE,
        1 => IDX_MOVE_FRAIL_ATTACK,
        n if move_history[n - 1] as usize == IDX_MOVE_SLAM => IDX_MOVE_HARDEN,
        _ => IDX_MOVE_SLAM,
    }
}
