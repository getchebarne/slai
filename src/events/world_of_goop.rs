use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::DeltaSign;

// Gather
const OPTION_GATHER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(11),
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(75),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const fn leave(min: u16, max: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Range { min, max },
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_LEAVE_BASE: [Effect; 2] = leave(20, 50);
static OPTION_LEAVE_A15: [Effect; 2] = leave(35, 75);

const LABELS_BASE: &[&str] = &[
    "[Gather Gold] Gain 75 Gold. Lose 11 HP.",
    "[Leave It] Lose 20-50 Gold.",
];
const LABELS_A15: &[&str] = &[
    "[Gather Gold] Gain 75 Gold. Lose 11 HP.",
    "[Leave It] Lose 35-75 Gold.",
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
        0 => OPTION_GATHER,
        1 if ascension < 15 => &OPTION_LEAVE_BASE,
        1 => &OPTION_LEAVE_A15,
        _ => unreachable!("world of goop option out of range: {idx}"),
    });
}
