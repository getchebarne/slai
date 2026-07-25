use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::CardName;
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
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_PRAY_BASE: [Effect; 2] = pray(100);
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
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Regret,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option("[Pray] Gain 100 Gold.", &OPTION_PRAY_BASE),
    make_entity_event_option(
        "[Desecrate] Gain 275 Gold. Become Cursed - Regret.",
        OPTION_DESECRATE,
    ),
    make_entity_event_option("[Leave] Nothing happens.", OPTION_LEAVE),
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option("[Pray] Gain 50 Gold.", &OPTION_PRAY_A15),
    make_entity_event_option(
        "[Desecrate] Gain 275 Gold. Become Cursed - Regret.",
        OPTION_DESECRATE,
    ),
    make_entity_event_option("[Leave] Nothing happens.", OPTION_LEAVE),
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
