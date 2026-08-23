use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::opt;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::RelicName;

// Take
const OPTION_TAKE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::GoldenIdol,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::EventAdvanceState { delta: 1 }, // Outrun / Smash / Hide
        id_source: None,
        target: Target::Direct(None),
    },
];

// Leave
// Outrun
const OPTION_OUTRUN: &[Effect] = &[
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Injury,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Smash: 25% -> 35% max HP loss at A15
const fn smash(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator,
                    denominator,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_SMASH_BASE: [Effect; 2] = smash(1, 4);
const OPTION_SMASH_A15: [Effect; 2] = smash(35, 100);

// Hide: 8% -> 10% max HP cap loss at A15
const fn hide(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator,
                    denominator,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_HIDE_BASE: [Effect; 2] = hide(8, 100);
const OPTION_HIDE_A15: [Effect; 2] = hide(10, 100);

static OPTIONS_BASE: &[&[Effect]] = &[
    opt(OPTION_TAKE),
    OPTION_LEAVE,
    opt(OPTION_OUTRUN),
    opt(&OPTION_SMASH_BASE),
    opt(&OPTION_HIDE_BASE),
];
static OPTIONS_A15: &[&[Effect]] = &[
    opt(OPTION_TAKE),
    OPTION_LEAVE,
    opt(OPTION_OUTRUN),
    opt(&OPTION_SMASH_A15),
    opt(&OPTION_HIDE_A15),
];

pub fn options(ascension: u8) -> &'static [&'static [Effect]] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}

pub fn option_available(stage: u8, idx: usize) -> bool {
    match idx {
        0 | 1 => stage == 0,
        2..=4 => stage == 1,
        _ => unreachable!("Golden idol option out of range: {idx}"),
    }
}
