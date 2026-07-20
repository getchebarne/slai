use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
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
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_TOUCH_BASE: [Effect; 3] = touch(75);
static OPTION_TOUCH_A15: [Effect; 3] = touch(50);

// Trade: gain random unowned face relic
const OPTION_TRADE: &[Effect] = &[
    Effect {
        kind: EffectKind::FaceTrade,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

const LABELS_BASE: &[&str] = &[
    "[Touch] Lose HP equal to 10% of Max HP. Gain 75 Gold.",
    "[Trade] Obtain a random face.",
    "[Leave] Nothing happens.",
];
const LABELS_A15: &[&str] = &[
    "[Touch] Lose HP equal to 10% of Max HP. Gain 50 Gold.",
    "[Trade] Obtain a random face.",
    "[Leave] Nothing happens.",
];

pub fn labels(ascension: u8) -> &'static [&'static str] {
    if ascension < 15 {
        LABELS_BASE
    } else {
        LABELS_A15
    }
}

pub fn push_option_effects(buf: &mut Vec<Effect>, ascension: u8, idx: usize) {
    buf.extend_from_slice(match idx {
        0 if ascension < 15 => &OPTION_TOUCH_BASE,
        0 => &OPTION_TOUCH_A15,
        1 => OPTION_TRADE,
        2 => OPTION_LEAVE,
        _ => unreachable!("face trader option out of range: {idx}"),
    });
}
