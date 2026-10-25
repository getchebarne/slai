use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::TARGET_CHARACTER;
use crate::effect::TARGET_SOURCE;
use crate::effect::Target;
use crate::entity::Intent;
use crate::entity::Move;
use crate::modifier::ModifierKind;
use crate::modifier::Modifiers;
use crate::modifier::has_modifier;
use crate::monsters::MonsterTemplate;
use crate::monsters::make_move;
use crate::monsters::modifier_fixed;
use crate::monsters::move_attack;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::MonsterKind;
use crate::types::MonsterName;
use rand::Rng;

static MOVE_SLASH: Move = move_attack("Slash", 20, 1);
static MOVE_SOUL_STRIKE: Move = move_attack("Soul Strike", 6, 4);

// Rebirth: the Awakened One leaves half-dead as its second form, then heals by its max HP
static MOVE_REBIRTH: Move = make_move(
    "Rebirth",
    &[
        Effect {
            kind: EffectKind::HalfDeadRebirth,
            id_source: None,
            target: TARGET_SOURCE,
        },
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
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
    Intent::Unknown,
);
static MOVE_DARK_ECHO: Move = move_attack("Dark Echo", 40, 1);

// Sludge: the hit, then a Void into the draw pile, never on top
static MOVE_SLUDGE: Move = make_move(
    "Sludge",
    &[
        Effect {
            kind: EffectKind::DamagePhysical {
                amount: 18,
                instances: 1,
                lifesteal: false,
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Void,
                card_pile: CardPile::Draw,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ],
    Intent::AttackDebuff {
        damage: 18,
        instances: 1,
    },
);
static MOVE_TACKLE: Move = move_attack("Tackle", 10, 3);
static MOVES: [Move; 6] = [
    MOVE_SLASH,
    MOVE_SOUL_STRIKE,
    MOVE_REBIRTH,
    MOVE_DARK_ECHO,
    MOVE_SLUDGE,
    MOVE_TACKLE,
];

const IDX_MOVE_SLASH: usize = 0;
const IDX_MOVE_SOUL_STRIKE: usize = 1;
pub const IDX_MOVE_REBIRTH: usize = 2;
const IDX_MOVE_DARK_ECHO: usize = 3;
const IDX_MOVE_SLUDGE: usize = 4;
const IDX_MOVE_TACKLE: usize = 5;

pub static AWAKENED_ONE: MonsterTemplate = MonsterTemplate {
    name: MonsterName::AwakenedOne,
    kind: MonsterKind::Boss,
    health_tiers: &[(0, (300, 300)), (9, (320, 320))],
    block_tiers: &[],
    move_tiers: &[(0, &[&MOVES])],
    modifier_tiers: &[
        (
            0,
            &[
                modifier_fixed(ModifierKind::Regeneration, 10),
                modifier_fixed(ModifierKind::Curiosity, 1),
                modifier_fixed(ModifierKind::Unawakened, 1),
            ],
        ),
        (
            4,
            &[
                modifier_fixed(ModifierKind::Regeneration, 10),
                modifier_fixed(ModifierKind::Curiosity, 1),
                modifier_fixed(ModifierKind::Unawakened, 1),
                modifier_fixed(ModifierKind::Strength, 2),
            ],
        ),
        (
            19,
            &[
                modifier_fixed(ModifierKind::Regeneration, 15),
                modifier_fixed(ModifierKind::Curiosity, 2),
                modifier_fixed(ModifierKind::Unawakened, 1),
                modifier_fixed(ModifierKind::Strength, 2),
            ],
        ),
    ],
};

// The first form, while it holds Unawakened, opens with Slash; the second opens with Dark Echo once Rebirth is its move
pub fn get_next_move_awakened_one(
    move_current: Option<usize>,
    move_history: &[u8],
    modifiers: &Modifiers,
    rng: &mut impl Rng,
) -> usize {
    if has_modifier(modifiers, ModifierKind::Unawakened) {
        if move_current.is_none() {
            return IDX_MOVE_SLASH;
        }

        // Soul Strike never twice in a row, Slash at most twice
        if rng.random_range(0..=99) < 25 {
            if move_history.last().copied() == Some(IDX_MOVE_SOUL_STRIKE as u8) {
                IDX_MOVE_SLASH
            } else {
                IDX_MOVE_SOUL_STRIKE
            }
        } else if move_history.ends_with(&[IDX_MOVE_SLASH as u8, IDX_MOVE_SLASH as u8]) {
            IDX_MOVE_SOUL_STRIKE
        } else {
            IDX_MOVE_SLASH
        }
    } else if move_current == Some(IDX_MOVE_REBIRTH) {
        IDX_MOVE_DARK_ECHO
    } else if rng.random_range(0..=99) < 50 {
        // Sludge and Tackle at most twice in a row each
        if move_history.ends_with(&[IDX_MOVE_SLUDGE as u8, IDX_MOVE_SLUDGE as u8]) {
            IDX_MOVE_TACKLE
        } else {
            IDX_MOVE_SLUDGE
        }
    } else if move_history.ends_with(&[IDX_MOVE_TACKLE as u8, IDX_MOVE_TACKLE as u8]) {
        IDX_MOVE_SLUDGE
    } else {
        IDX_MOVE_TACKLE
    }
}
