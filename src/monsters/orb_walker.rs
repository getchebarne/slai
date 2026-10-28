use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

// Laser: the hit, then one Burn into the draw pile and one on top of the discard pile
const fn move_laser(damage: u16) -> Move {
    make_move(
        "Laser",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    instances: 1,
                    lifesteal: false,
                    if_poisoned: false,
                    scaled_at: ScaledAt::Now,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Burn,
                    card_pile: CardPile::Draw,
                    count: 1,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            },
            Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Burn,
                    card_pile: CardPile::Discard,
                    count: 1,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            },
        ],
        Intent::AttackDebuff {
            damage,
            instances: 1,
        },
    )
}

static MOVE_LASER_10: Move = move_laser(10);
static MOVE_LASER_11: Move = move_laser(11);
static MOVE_CLAW_15: Move = move_attack("Claw", 15, 1);
static MOVE_CLAW_16: Move = move_attack("Claw", 16, 1);
static MOVES_ASC0: [Move; 2] = [MOVE_LASER_10, MOVE_CLAW_15];
static MOVES_ASC2: [Move; 2] = [MOVE_LASER_11, MOVE_CLAW_16];

const IDX_MOVE_LASER: usize = 0;
const IDX_MOVE_CLAW: usize = 1;

pub static ORB_WALKER: MonsterTemplate = MonsterTemplate {
    name: MonsterName::OrbWalker,
    kind: MonsterKind::Normal,
    health_tiers: &[(0, (90, 96)), (7, (92, 102))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES_ASC0]), (2, &[&MOVES_ASC2])],
    modifier_tiers: &[
        (0, &[modifier_fixed(ModifierKind::StrengthUp, 3)]),
        (17, &[modifier_fixed(ModifierKind::StrengthUp, 5)]),
    ],
};

pub fn get_next_move_orb_walker(move_history: &[u8], rng: &mut impl Rng) -> usize {
    let roll = rng.random_range(0..=99);
    if roll < 40 {
        if move_history.ends_with(&[IDX_MOVE_CLAW as u8, IDX_MOVE_CLAW as u8]) {
            IDX_MOVE_LASER
        } else {
            IDX_MOVE_CLAW
        }
    } else if move_history.ends_with(&[IDX_MOVE_LASER as u8, IDX_MOVE_LASER as u8]) {
        IDX_MOVE_CLAW
    } else {
        IDX_MOVE_LASER
    }
}
