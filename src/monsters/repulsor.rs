use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::move_attack;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_DAZE: Move = make_move(
    "Daze",
    &[Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Dazed,
            card_pile: CardPile::Draw,
            count: 2,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    }],
    Intent::Debuff,
);
static MOVE_ATTACK_11: Move = move_attack("Attack", 11, 1);
static MOVE_ATTACK_13: Move = move_attack("Attack", 13, 1);

static MOVES_ASC0: [Move; 2] = [MOVE_DAZE, MOVE_ATTACK_11];
static MOVES_ASC2: [Move; 2] = [MOVE_DAZE, MOVE_ATTACK_13];

const IDX_MOVE_DAZE: usize = 0;
const IDX_MOVE_ATTACK: usize = 1;

pub static REPULSOR: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Repulsor,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (29, 35)), (7, (31, 38))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[],
};

// One roll: Attack below 20 unless it was the last move, Daze otherwise
pub fn get_next_move_repulsor(move_history: &[u8], rng: &mut impl Rng) -> usize {
    let roll = rng.random_range(0..=99);
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    if roll < 20 && move_last != Some(IDX_MOVE_ATTACK) {
        IDX_MOVE_ATTACK
    } else {
        IDX_MOVE_DAZE
    }
}
