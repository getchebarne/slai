use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::RelicName;

// Take
const OPTION_TAKE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::GoldenIdol,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::EventAdvanceState { delta: 1 }, // Outrun / Smash / Hide
        id_source: None,
        target: Target::Direct(None),
    },
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// Outrun
const OPTION_OUTRUN: &[Effect] = &[
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Injury,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Smash: 25% -> 35% max HP loss at A15
const fn smash(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator,
                    denominator,
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
const OPTION_SMASH_BASE: [Effect; 2] = smash(1, 4);
const OPTION_SMASH_A15: [Effect; 2] = smash(35, 100);

// Hide: 8% -> 10% max HP cap loss at A15
const fn hide(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Relative {
                    numerator,
                    denominator,
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
const OPTION_HIDE_BASE: [Effect; 2] = hide(8, 100);
const OPTION_HIDE_A15: [Effect; 2] = hide(10, 100);

const OPTIONS_BASE: &[(&str, &[Effect])] = &[
    ("[Take] Obtain Golden Idol.", OPTION_TAKE),
    ("[Leave] Nothing happens.", OPTION_LEAVE),
    ("[Outrun] Become Cursed - Injury.", OPTION_OUTRUN),
    (
        "[Smash] Take 25% of your max HP as damage.",
        &OPTION_SMASH_BASE,
    ),
    ("[Hide] Lose 8% of your max HP.", &OPTION_HIDE_BASE),
];
const OPTIONS_A15: &[(&str, &[Effect])] = &[
    ("[Take] Obtain Golden Idol.", OPTION_TAKE),
    ("[Leave] Nothing happens.", OPTION_LEAVE),
    ("[Outrun] Become Cursed - Injury.", OPTION_OUTRUN),
    (
        "[Smash] Take 35% of your max HP as damage.",
        &OPTION_SMASH_A15,
    ),
    ("[Hide] Lose 10% of your max HP.", &OPTION_HIDE_A15),
];

pub fn options(ascension: u8) -> &'static [(&'static str, &'static [Effect])] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}

pub fn option_available(stage: u8, idx: usize) -> bool {
    match idx {
        0 | 1 => stage == 0,
        2..=4 => stage == 1,
        _ => unreachable!("golden idol option out of range: {idx}"),
    }
}
