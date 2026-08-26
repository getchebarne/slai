use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;

// Agree: -25 gold gain at A15
const fn agree(gold: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(gold),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Doubt,
                pile: CardPile::Deck,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Agree: 175 gold for a Curse of the Bell
const OPTION_AGREE_BASE: [Effect; 3] = agree(175);

// Agree at A15+: only 150 gold
const OPTION_AGREE_A15: [Effect; 3] = agree(150);

// Disagree
const OPTION_DISAGREE: &[Effect] = &[EFFECT_EVENT_CONSUME];

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_AGREE_BASE),
    make_event_option_template(OPTION_DISAGREE),
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_AGREE_A15),
    make_event_option_template(OPTION_DISAGREE),
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
