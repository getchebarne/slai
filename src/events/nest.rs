use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
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

const fn take_gold(gold: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(gold),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Damage lands before the dagger, as in the source
const OPTION_DAGGER: [Effect; 3] = [
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(6),
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::RitualDagger,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Take the gold: 99
const OPTION_GOLD_BASE: [Effect; 2] = take_gold(99);

// Take the gold at A15+: only 50
const OPTION_GOLD_A15: [Effect; 2] = take_gold(50);

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_GOLD_BASE),
    make_event_option_template(&OPTION_DAGGER),
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(&OPTION_GOLD_A15),
    make_event_option_template(&OPTION_DAGGER),
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
