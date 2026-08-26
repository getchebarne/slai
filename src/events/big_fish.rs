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
    EFFECT_EVENT_CONSUME,
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
    EFFECT_EVENT_CONSUME,
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
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_BANANA),
    make_event_option_template(OPTION_DONUT),
    make_event_option_template(OPTION_BOX),
];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
