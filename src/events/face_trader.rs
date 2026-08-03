use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::types::DeltaSign;

// Touch: gold gain first, then health loss; -25 gold gain at A15
const fn touch(gold: u16) -> [Effect; 3] {
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
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator: 1,
                    denominator: 10,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_TOUCH_BASE: [Effect; 3] = touch(75);
const OPTION_TOUCH_A15: [Effect; 3] = touch(50);

// Trade: gain random unowned face Relic
const OPTION_TRADE: &[Effect] = &[
    Effect {
        kind: EffectKind::FaceTrade,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option(
        "[Touch] Lose HP equal to 10% of Max HP. Gain 75 Gold.",
        &OPTION_TOUCH_BASE,
    ),
    make_entity_event_option("[Trade] Obtain a random face.", OPTION_TRADE),
    OPTION_LEAVE,
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option(
        "[Touch] Lose HP equal to 10% of Max HP. Gain 50 Gold.",
        &OPTION_TOUCH_A15,
    ),
    make_entity_event_option("[Trade] Obtain a random face.", OPTION_TRADE),
    OPTION_LEAVE,
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
