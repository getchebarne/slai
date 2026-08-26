use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
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
    EFFECT_EVENT_CONSUME,
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
        EFFECT_EVENT_CONSUME,
    ]
}

// Smash the trap: light damage below A15
const OPTION_SMASH_BASE: [Effect; 2] = smash(1, 4);

// Smash at A15+: 35% max HP
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
        EFFECT_EVENT_CONSUME,
    ]
}

// Hide: 8% max HP
const OPTION_HIDE_BASE: [Effect; 2] = hide(8, 100);

// Hide at A15+: 10% max HP
const OPTION_HIDE_A15: [Effect; 2] = hide(10, 100);

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_TAKE),
    EOT_LEAVE,
    make_event_option_template(OPTION_OUTRUN),
    make_event_option_template(&OPTION_SMASH_BASE),
    make_event_option_template(&OPTION_HIDE_BASE),
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_TAKE),
    EOT_LEAVE,
    make_event_option_template(OPTION_OUTRUN),
    make_event_option_template(&OPTION_SMASH_A15),
    make_event_option_template(&OPTION_HIDE_A15),
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn option_available(state: &GameState, idx: usize) -> bool {
    let stage = state.event.stage;
    match idx {
        0 | 1 => stage == 0,
        2..=4 => stage == 1,
        _ => unreachable!("Golden idol option out of range: {idx}"),
    }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
