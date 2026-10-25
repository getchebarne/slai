use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_MONSTERS_ALL;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::MonsterKind;
use crate::types::MonsterName;

// Beam: two hits, then 2 Dazed into the discard pile
const fn move_beam(damage: u16) -> Move {
    make_move(
        "Beam",
        &[
            Effect {
                kind: EffectKind::DamagePhysical {
                    amount: damage,
                    instances: 2,
                    lifesteal: false,
                },
                id_source: None,
                target: TARGET_CHARACTER,
            },
            Effect {
                kind: EffectKind::CardAdd {
                    card_name: CardName::Dazed,
                    card_pile: CardPile::Discard,
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

static MOVE_BEAM_10: Move = move_beam(10);
static MOVE_BEAM_12: Move = move_beam(12);

// Square of Protection: 16 block to every living Monster, Deca included
static MOVE_SQUARE_OF_PROTECTION: Move = make_move(
    "Square of Protection",
    &[Effect {
        kind: EffectKind::BlockGain { amount: 16 },
        id_source: None,
        target: TARGET_MONSTERS_ALL,
    }],
    Intent::Block,
);

// The A19 Square: the block, then 3 Plated Armor to every living Monster
static MOVE_SQUARE_OF_PROTECTION_PLATED_ARMOR: Move = make_move(
    "Square of Protection",
    &[
        Effect {
            kind: EffectKind::BlockGain { amount: 16 },
            id_source: None,
            target: TARGET_MONSTERS_ALL,
        },
        Effect {
            kind: EffectKind::ModifierDelta {
                kind: ModifierKind::PlatedArmor,
                stacks: 3,
            },
            id_source: None,
            target: TARGET_MONSTERS_ALL,
        },
    ],
    Intent::BlockBuff,
);

static MOVES_ASC0: [Move; 2] = [MOVE_BEAM_10, MOVE_SQUARE_OF_PROTECTION];
static MOVES_ASC4: [Move; 2] = [MOVE_BEAM_12, MOVE_SQUARE_OF_PROTECTION];
static MOVES_ASC19: [Move; 2] = [MOVE_BEAM_12, MOVE_SQUARE_OF_PROTECTION_PLATED_ARMOR];

const IDX_MOVE_BEAM: usize = 0;
const IDX_MOVE_SQUARE_OF_PROTECTION: usize = 1;

pub static DECA: MonsterTemplate = MonsterTemplate {
    name: MonsterName::Deca,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (250, 250)), (9, (265, 265))],
    block_tiers: &[],
    move_tiers: &[
        (0, &[&MOVES_ASC0]),
        (4, &[&MOVES_ASC4]),
        (19, &[&MOVES_ASC19]),
    ],
    modifier_tiers: &[
        (0, &[modifier_fixed(ModifierKind::Artifact, 2)]),
        (19, &[modifier_fixed(ModifierKind::Artifact, 3)]),
    ],
};

// Strict alternation that opens with Beam
pub fn get_next_move_deca(move_current: Option<usize>) -> usize {
    if move_current == Some(IDX_MOVE_BEAM) {
        IDX_MOVE_SQUARE_OF_PROTECTION
    } else {
        IDX_MOVE_BEAM
    }
}
