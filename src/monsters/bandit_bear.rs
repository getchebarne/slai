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
use crate::monsters::move_debuff;
use crate::types::MonsterKind;
use crate::types::MonsterName;

// Lunge: the hit plus 9 self block
const fn move_lunge(damage: u16) -> Move {
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

static MOVE_BEAR_HUG_2: Move = move_debuff("Bear Hug", ModifierKind::Dexterity, -2, Intent::Debuff);
static MOVE_BEAR_HUG_4: Move = move_debuff("Bear Hug", ModifierKind::Dexterity, -4, Intent::Debuff);
static MOVE_LUNGE_9: Move = move_lunge(9);
static MOVE_LUNGE_10: Move = move_lunge(10);
static MOVE_MAUL_18: Move = move_attack("Maul", 18, 1);
static MOVE_MAUL_20: Move = move_attack("Maul", 20, 1);

static MOVES_ASC0: [Move; 3] = [MOVE_BEAR_HUG_2, MOVE_LUNGE_9, MOVE_MAUL_18];
static MOVES_ASC2: [Move; 3] = [MOVE_BEAR_HUG_2, MOVE_LUNGE_10, MOVE_MAUL_20];
static MOVES_ASC17: [Move; 3] = [MOVE_BEAR_HUG_4, MOVE_LUNGE_10, MOVE_MAUL_20];

const IDX_MOVE_BEAR_HUG: usize = 0;
const IDX_MOVE_LUNGE: usize = 1;
const IDX_MOVE_MAUL: usize = 2;

pub static TEMPLATE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::BanditBear,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (38, 42)), (7, (40, 44))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

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
