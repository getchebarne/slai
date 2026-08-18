use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventOptionTemplate;
use crate::events::make_event_option_template;
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
        EVENT_CONSUME_EFFECT,
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
    EVENT_CONSUME_EFFECT,
];

const OPTION_GOLD_BASE: [Effect; 2] = take_gold(99);
const OPTION_GOLD_A15: [Effect; 2] = take_gold(50);

static OPTIONS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template("[Smash and Grab] Gain 99 Gold.", &OPTION_GOLD_BASE),
    make_event_option_template(
        "[Join the Cult] Lose 6 HP. Obtain Ritual Dagger.",
        &OPTION_DAGGER,
    ),
];
static OPTIONS_A15: &[EventOptionTemplate] = &[
    make_event_option_template("[Smash and Grab] Gain 50 Gold.", &OPTION_GOLD_A15),
    make_event_option_template(
        "[Join the Cult] Lose 6 HP. Obtain Ritual Dagger.",
        &OPTION_DAGGER,
    ),
];

pub fn options(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
