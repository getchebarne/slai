use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
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

// Pray: -50 gold gain at A15
const fn pray(amount: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(amount),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Pray: 100 gold
const OPTION_PRAY_BASE: [Effect; 2] = pray(100);

// Pray at A15+: only 50 gold
const OPTION_PRAY_A15: [Effect; 2] = pray(50);

// Desecrate
const OPTION_DESECRATE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(275),
        },
        id_source: None,
        target: Target::Direct(None),
    },
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
    EFFECT_EVENT_CONSUME,
];

// Leave
static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_PRAY_BASE),
    make_event_option_template(OPTION_DESECRATE),
    EOT_LEAVE,
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_PRAY_A15),
    make_event_option_template(OPTION_DESECRATE),
    EOT_LEAVE,
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
