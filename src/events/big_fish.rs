use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;

// Banana
const OPTION_BANANA: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 3,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EVENT_CONSUME_EFFECT,
];

// Donut
const OPTION_DONUT: &[Effect] = &[
    Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(5),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EVENT_CONSUME_EFFECT,
];

// Box
const OPTION_BOX: &[Effect] = &[
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Regret,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::RelicGrantRandom { tier: None },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[&[Effect]] = &[opt(OPTION_BANANA), opt(OPTION_DONUT), opt(OPTION_BOX)];
