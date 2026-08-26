use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_SOURCE;
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

static MOVE_MUG_10: Move = move_attack("Mug", 10, 1);
static MOVE_MUG_11: Move = move_attack("Mug", 11, 1);
static MOVE_LUNGE_12: Move = move_attack("Lunge", 12, 1);
static MOVE_LUNGE_14: Move = move_attack("Lunge", 14, 1);
static MOVE_SMOKE_BOMB: Move = make_move(
    "Smoke Bomb",
    &[Effect {
        kind: EffectKind::BlockGain { amount: 6 },
        id_source: None,
        target: TARGET_SOURCE,
    }],
    Intent::Block,
);
static MOVE_ESCAPE: Move = make_move(
    "Escape",
    &[Effect {
        kind: EffectKind::MonsterEscape,
        id_source: None,
        target: TARGET_SOURCE,
    }],
    Intent::Escape,
);

static MOVES_ASC0: [Move; 4] = [MOVE_MUG_10, MOVE_LUNGE_12, MOVE_SMOKE_BOMB, MOVE_ESCAPE];
static MOVES_ASC2: [Move; 4] = [MOVE_MUG_11, MOVE_LUNGE_14, MOVE_SMOKE_BOMB, MOVE_ESCAPE];

const IDX_MOVE_MUG: usize = 0;
const IDX_MOVE_LUNGE: usize = 1;
const IDX_MOVE_SMOKE_BOMB: usize = 2;
const IDX_MOVE_ESCAPE: usize = 3;

pub static LOOTER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Looter,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (44, 48)), (7, (46, 50))],
    block_start: 0,
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[
        (0, &[modifier_fixed(ModifierKind::Thievery, 15)]),
        (17, &[modifier_fixed(ModifierKind::Thievery, 20)]),
    ],
};

pub fn get_next_move_looter(
    move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_MUG;
    }
    let last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    match last {
        IDX_MOVE_MUG => {
            // Count of Mug + Lunge in history (the "slash_count")
            let slash_count = move_history
                .iter()
                .filter(|&&m| m == IDX_MOVE_MUG as u8 || m == IDX_MOVE_LUNGE as u8)
                .count();
            if slash_count < 2 {
                IDX_MOVE_MUG
            } else if rng.random_bool(0.5) {
                IDX_MOVE_SMOKE_BOMB
            } else {
                IDX_MOVE_LUNGE
            }
        }
        IDX_MOVE_LUNGE => IDX_MOVE_SMOKE_BOMB,
        IDX_MOVE_SMOKE_BOMB => IDX_MOVE_ESCAPE,
        IDX_MOVE_ESCAPE => IDX_MOVE_ESCAPE,
        _ => unreachable!("Looter unexpected move idx: {last}"),
    }
}
