use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::entity::Intent;
use crate::entity::Move;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::monsters::move_block_buff;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_CHOMP_11: Move = move_attack("Chomp", 11, 1);
static MOVE_CHOMP_12: Move = move_attack("Chomp", 12, 1);
static MOVE_THRASH: Move = make_move(
    "Thrash",
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
            kind: EffectKind::BlockGain { amount: 5 },
            id_source: None,
            target: TARGET_SOURCE,
        },
    ],
    Intent::AttackBlock {
        damage: 7,
        instances: 1,
    },
);
static MOVE_BELLOW_3_6: Move = move_block_buff("Bellow", 6, 3);
static MOVE_BELLOW_4_6: Move = move_block_buff("Bellow", 6, 4);
static MOVE_BELLOW_5_9: Move = move_block_buff("Bellow", 9, 5);
static MOVES_ASC0: [Move; 3] = [MOVE_CHOMP_11, MOVE_BELLOW_3_6, MOVE_THRASH];
static MOVES_ASC2: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_4_6, MOVE_THRASH];
static MOVES_ASC17: [Move; 3] = [MOVE_CHOMP_12, MOVE_BELLOW_5_9, MOVE_THRASH];

const IDX_MOVE_CHOMP: usize = 0;
const IDX_MOVE_BELLOW: usize = 1;
const IDX_MOVE_THRASH: usize = 2;

pub static TEMPLATE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::JawWorm,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (40, 44)), (7, (42, 46))],
    block_start: 0,
    move_tiers: &[(0, &MOVES_ASC0), (2, &MOVES_ASC2), (17, &MOVES_ASC17)],
    modifier_tiers: &[],
};

pub fn get_next_move_jaw_worm(
    move_current: Option<usize>,
    move_history: &[u8],
    rng: &mut impl Rng,
) -> usize {
    if move_current.is_none() {
        return IDX_MOVE_CHOMP;
    }

    let roll = rng.random_range(0..=99);
    let move_last = *move_history
        .last()
        .expect("`move_history` cannot be empty here") as usize;

    if roll < 25 {
        if move_last == IDX_MOVE_CHOMP {
            return if rng.random_bool(0.5625) {
                IDX_MOVE_BELLOW
            } else {
                IDX_MOVE_THRASH
            };
        }
        IDX_MOVE_CHOMP
    } else if roll < 55 {
        if move_history.ends_with(&[IDX_MOVE_THRASH as u8, IDX_MOVE_THRASH as u8]) {
            return if rng.random_bool(0.357) {
                IDX_MOVE_CHOMP
            } else {
                IDX_MOVE_BELLOW
            };
        }
        IDX_MOVE_THRASH
    } else {
        if move_last == IDX_MOVE_BELLOW {
            return if rng.random_bool(0.416) {
                IDX_MOVE_CHOMP
            } else {
                IDX_MOVE_THRASH
            };
        }
        IDX_MOVE_BELLOW
    }
}
