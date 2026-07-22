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
const OPTION_LEAVE_BASE: [Effect; 2] = leave(20, 50);
const OPTION_LEAVE_A15: [Effect; 2] = leave(35, 75);

const OPTIONS_BASE: &[(&str, &[Effect])] = &[
    ("[Gather Gold] Gain 75 Gold. Lose 11 HP.", OPTION_GATHER),
    ("[Leave It] Lose 20-50 Gold.", &OPTION_LEAVE_BASE),
];
const OPTIONS_A15: &[(&str, &[Effect])] = &[
    ("[Gather Gold] Gain 75 Gold. Lose 11 HP.", OPTION_GATHER),
    ("[Leave It] Lose 35-75 Gold.", &OPTION_LEAVE_A15),
];

pub fn options(ascension: u8) -> &'static [(&'static str, &'static [Effect])] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
