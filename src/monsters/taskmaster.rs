use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack_card_add;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;

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
    move_attack_card_add("Scouring Whip", 7, CardName::Wound, 1, false);
static MOVE_SCOURING_WHIP_W2: Move =
    move_attack_card_add("Scouring Whip", 7, CardName::Wound, 2, false);

static MOVES_ASC0: [Move; 1] = [MOVE_SCOURING_WHIP_W1];
static MOVES_ASC3: [Move; 1] = [MOVE_SCOURING_WHIP_W2];
static MOVES_ASC18: [Move; 1] = [MOVE_SCOURING_WHIP_A18];

pub static TASKMASTER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Taskmaster,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (54, 60)), (8, (57, 64))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (3, &MOVES_ASC3), (18, &MOVES_ASC18)],
    modifier_tiers: &[],
};

// Doesn't have an AI: always uses Scouring Whip
