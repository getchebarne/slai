use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CLOSE;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

// Don: 222 gold, and the Red Mask is kept
const OPTION_DON: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(222),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Offer: every coin held buys the Red Mask, allowed at 0 gold
const OPTION_OFFER: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 1,
                rounding: Rounding::Truncate,
                read_at: ReadAt::Now,
            },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::RedMask,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_DON),
    make_event_option_template(OPTION_OFFER),
    // Leave goes to the map at once
    make_event_option_template(&[EFFECT_EVENT_CLOSE]),
];

// Don is open only with the Red Mask, Offer only without it
pub fn option_available(state: &GameState, idx: usize) -> bool {
    let owned = has_relic(&state.id_relics, RelicName::RedMask);
    match idx {
        0 => owned,
        1 => !owned,
        _ => true,
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
