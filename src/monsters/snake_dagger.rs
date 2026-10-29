use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::ScaledAt;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack_card_add;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::MonsterKind;
use crate::types::MonsterName;

static MOVE_STAB: Move = move_attack_card_add("Stab", 9, CardName::Wound, 1, false);

// Explode: the hit, then the dagger loses its whole health
static MOVE_EXPLODE: Move = make_move(
    "Explode",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 25,
                instances: 1,
                lifesteal: false,
                if_poisoned: false,
                scaled_at: ScaledAt::Now,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 1,
                    rounding: Rounding::Truncate,
                    read_at: ReadAt::Now,
                },
            },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::Attack {
        damage: 25,
        instances: 1,
    },
);

static MOVES: [Move; 2] = [MOVE_STAB, MOVE_EXPLODE];

const IDX_MOVE_STAB: usize = 0;
const IDX_MOVE_EXPLODE: usize = 1;

pub static SNAKE_DAGGER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SnakeDagger,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (20, 25))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES])],
    modifier_tiers: &[(0, &[modifier_fixed(ModifierKind::Minion, 1)])],
};

// Stab first, Explode after
pub fn get_next_move_snake_dagger(move_history: &[u8]) -> usize {
    if move_history.is_empty() {
        IDX_MOVE_STAB
    } else {
        IDX_MOVE_EXPLODE
    }
}
