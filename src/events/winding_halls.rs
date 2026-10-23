use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;

// Embrace Madness: 12.5% -> 18% max HP loss at A15, then two Madness
const fn embrace_madness(numerator: u8, denominator: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator,
                    denominator,
                    rounding: Rounding::HalfUp,
                    read_at: ReadAt::EventOpen,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Madness,
                card_pile: CardPile::Deck,
                count: 2,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Writhe: 25% -> 20% max HP heal at A15, then a Writhe
const fn writhe(numerator: u8, denominator: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Relative {
                    numerator,
                    denominator,
                    rounding: Rounding::HalfUp,
                    read_at: ReadAt::EventOpen,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Writhe,
                card_pile: CardPile::Deck,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Max HP: 5% max HP cap loss at every ascension; it rounds to 0 below 10 max HP
const OPTION_MAX_HP: [Effect; 2] = [
    Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 20,
                rounding: Rounding::HalfUp,
                read_at: ReadAt::EventOpen,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EFFECT_EVENT_CONSUME,
];

// Embrace Madness: 12.5% max HP
const OPTION_EMBRACE_MADNESS_BASE: [Effect; 3] = embrace_madness(1, 8);

// Embrace Madness at A15+: 18% max HP
const OPTION_EMBRACE_MADNESS_A15: [Effect; 3] = embrace_madness(18, 100);

// Writhe: heal 25% max HP
const OPTION_WRITHE_BASE: [Effect; 3] = writhe(1, 4);

// Writhe at A15+: only 20%
const OPTION_WRITHE_A15: [Effect; 3] = writhe(1, 5);

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_EMBRACE_MADNESS_BASE),
    make_event_option_template(&OPTION_WRITHE_BASE),
    make_event_option_template(&OPTION_MAX_HP),
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_EMBRACE_MADNESS_A15),
    make_event_option_template(&OPTION_WRITHE_A15),
    make_event_option_template(&OPTION_MAX_HP),
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
