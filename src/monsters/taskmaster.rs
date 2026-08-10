use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::ZERO_MODIFIERS;
use crate::monsters::make_entity_monster;
use crate::monsters::make_move;
use crate::monsters::make_move_attack_card_add;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use crate::types::Vitals;
use rand::Rng;

// A18+ Scouring Whip also ramps Strength every turn
static MOVE_SCOURING_WHIP_A18: Move = make_move(
    "Scouring Whip",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 7,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Wound,
                pile: CardPile::Discard,
                count: 3,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 1,
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::AttackDebuff {
        damage: 7,
        instances: 1,
    },
);

static MOVE_SCOURING_WHIP_W1: Move =
    make_move_attack_card_add("Scouring Whip", 7, CardName::Wound, 1, false);
static MOVE_SCOURING_WHIP_W2: Move =
    make_move_attack_card_add("Scouring Whip", 7, CardName::Wound, 2, false);

static MOVES_ASC0: [Move; 1] = [MOVE_SCOURING_WHIP_W1];
static MOVES_ASC3: [Move; 1] = [MOVE_SCOURING_WHIP_W2];
static MOVES_ASC18: [Move; 1] = [MOVE_SCOURING_WHIP_A18];

pub fn spawn_monster_taskmaster(ascension_level: u8, rng: &mut impl Rng) -> Entity {
    let (health_max_min, health_max_max) = if ascension_level < 8 {
        (54, 60)
    } else {
        (57, 64)
    };
    let health_max = rng.random_range(health_max_min..=health_max_max);

    let moves: &'static [Move] = if ascension_level < 3 {
        &MOVES_ASC0
    } else if ascension_level < 18 {
        &MOVES_ASC3
    } else {
        &MOVES_ASC18
    };

    make_entity_monster(
        MonsterName::Taskmaster,
        MonsterKind::Elite,
        Vitals {
            health: health_max,
            health_max,
            block: 0,
        },
        ZERO_MODIFIERS,
        moves,
    )
}

// Doesn't have an AI: always uses Scouring Whip
