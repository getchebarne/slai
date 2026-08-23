use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;
use crate::types::CardColor;
use crate::types::CardPile;
use crate::types::CardRarity;
use crate::types::DeltaSign;

pub const KNOWING_SKULL_COST_START: u16 = 6;
pub const KNOWING_SKULL_GOLD: u16 = 90;
pub const KNOWING_SKULL_COST_LEAVE: u16 = 6;

const fn wish(reward: EffectKind) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(KNOWING_SKULL_COST_START),
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: reward,
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::KnowingSkullCostBump,
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}

const OPTION_POTION: [Effect; 3] = wish(EffectKind::PotionAddRandom { limited: false });
const OPTION_GOLD: [Effect; 3] = wish(EffectKind::GoldDelta {
    sign: DeltaSign::Gain,
    amount: Amount::Absolute(KNOWING_SKULL_GOLD),
});
const OPTION_CARD: [Effect; 3] = wish(EffectKind::CardAddRandom {
    color: CardColor::Colorless,
    kind: None,
    pile: CardPile::Deck,
    count: 1,
    cost_zero: None,
    upgraded: false,
    rarity: Some(CardRarity::Uncommon),
});

// Leaving costs a flat 6 HP
const OPTION_LEAVE_PAID: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(KNOWING_SKULL_COST_LEAVE),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[&[Effect]] = &[
    opt(&OPTION_POTION),
    opt(&OPTION_GOLD),
    opt(&OPTION_CARD),
    opt(OPTION_LEAVE_PAID),
];
