use crate::consts::MAX_MONSTER_MOVES;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ScaledAt;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTERS_ALL;
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

// Burn Strike: two hits, then 2 Burns into the discard pile, or on top of the draw pile from A18
const fn move_burn_strike(damage: u16, card_pile: CardPile) -> Move {
    make_move(
        "Burn Strike",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    instances: 2,
                    lifesteal: false,
                    if_poisoned: false,
                    block_from_hit: false,
                    scaled_at: ScaledAt::Now,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Burn,
                    card_pile,
                    count: 2,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            },
        ],
        Intent::AttackDebuff {
            damage,
            instances: 2,
        },
    )
}

// Piercer: 2 Strength to every living Monster, the Spear included
static MOVE_PIERCER: Move = make_move(
    "Piercer",
    &[Effect {
        kind: EffectKind::ModifierDelta {
            kind: ModifierKind::Strength,
            stacks: 2,
        },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    Intent::Buff,
);

static MOVE_BURN_STRIKE_5: Move = move_burn_strike(5, CardPile::Discard);
static MOVE_BURN_STRIKE_6: Move = move_burn_strike(6, CardPile::Discard);
static MOVE_BURN_STRIKE_6_DRAW_TOP: Move = move_burn_strike(6, CardPile::DrawTop);
static MOVE_SKEWER_3: Move = move_attack("Skewer", 10, 3);
static MOVE_SKEWER_4: Move = move_attack("Skewer", 10, 4);

static MOVES_ASC0: [Move; 3] = [MOVE_BURN_STRIKE_5, MOVE_PIERCER, MOVE_SKEWER_3];
static MOVES_ASC3: [Move; 3] = [MOVE_BURN_STRIKE_6, MOVE_PIERCER, MOVE_SKEWER_4];
static MOVES_ASC18: [Move; 3] = [MOVE_BURN_STRIKE_6_DRAW_TOP, MOVE_PIERCER, MOVE_SKEWER_4];

const IDX_MOVE_BURN_STRIKE: usize = 0;
const IDX_MOVE_PIERCER: usize = 1;
const IDX_MOVE_SKEWER: usize = 2;

pub static SPIRE_SPEAR: MonsterTemplate = MonsterTemplate {
    name: MonsterName::SpireSpear,
    kind: MonsterKind::Elite,
    health_tiers: &[(0, (160, 160)), (8, (180, 180))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (3, &[&MOVES_ASC3]),
        (18, &[&MOVES_ASC18]),
    ],
    modifier_tiers: &[
        (0, &[modifier_fixed(ModifierKind::Artifact, 1)]),
        (18, &[modifier_fixed(ModifierKind::Artifact, 2)]),
    ],
};

// A three-move cycle: Burn Strike, or Piercer right after a Burn Strike, then Skewer, then a coin picks Piercer or Burn Strike
pub fn get_next_move_spire_spear(
    move_history: &[u8],
    move_uses: &[u8; MAX_MONSTER_MOVES],
    rng: &mut impl Rng,
) -> usize {
    // The cycle's position counts executed moves, which outlast the history's 64-entry window
    let moves_used: usize = move_uses.iter().map(|&uses| usize::from(uses)).sum();
    let move_last = move_history
        .last()
        .copied()
        .map(|idx_move| idx_move as usize);
    match moves_used % 3 {
        0 => {
            if move_last == Some(IDX_MOVE_BURN_STRIKE) {
                IDX_MOVE_PIERCER
            } else {
                IDX_MOVE_BURN_STRIKE
            }
        }
        1 => IDX_MOVE_SKEWER,
        _ => {
            if rng.random_bool(0.5) {
                IDX_MOVE_PIERCER
            } else {
                IDX_MOVE_BURN_STRIKE
            }
        }
    }
}
