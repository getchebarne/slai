use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
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

// A17+ Tail Whip also applies Weak
static MOVE_TAIL_WHIP_10_A17: Move = make_move(
    "Tail Whip",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 10,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Vulnerable,
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
    Intent::AttackDebuff {
        damage: 10,
        instances: 1,
    },
);

static MOVE_GLARE: Move = move_debuff(
    "Perplexing Glare",
    ModifierKind::Confusion,
    1,
    Intent::DebuffPowerful,
);
static MOVE_BITE_15: Move = move_attack("Bite", 15, 1);
static MOVE_BITE_18: Move = move_attack("Bite", 18, 1);
static MOVE_TAIL_WHIP_8: Move = move_attack_debuff("Tail Whip", 8, ModifierKind::Vulnerable, 2);
static MOVE_TAIL_WHIP_10: Move = move_attack_debuff("Tail Whip", 10, ModifierKind::Vulnerable, 2);

static MOVES_ASC0: [Move; 3] = [MOVE_GLARE, MOVE_BITE_15, MOVE_TAIL_WHIP_8];
static MOVES_ASC2: [Move; 3] = [MOVE_GLARE, MOVE_BITE_18, MOVE_TAIL_WHIP_10];
static MOVES_ASC17: [Move; 3] = [MOVE_GLARE, MOVE_BITE_18, MOVE_TAIL_WHIP_10_A17];

const IDX_MOVE_GLARE: usize = 0;
const IDX_MOVE_BITE: usize = 1;
const IDX_MOVE_TAIL_WHIP: usize = 2;

pub static SNECKO: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Snecko,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (114, 120)), (7, (120, 125))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

pub fn get_next_move_snecko(
    move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_GLARE;
    }
    if rng.random_range(0..=99) < 40
        || move_history.ends_with(&[IDX_MOVE_BITE as u8, IDX_MOVE_BITE as u8])
    {
        IDX_MOVE_TAIL_WHIP
    } else {
        IDX_MOVE_BITE
    }
}
