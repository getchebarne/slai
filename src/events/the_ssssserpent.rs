use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::CardName;
use crate::types::DeltaSign;

// Agree: -25 gold gain at A15
const fn agree(gold: u16) -> [Effect; 3] {
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
            kind: EffectKind::CardAddToDeck {
                card_name: CardName::Doubt,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_AGREE_BASE: [Effect; 3] = agree(175);
static OPTION_AGREE_A15: [Effect; 3] = agree(150);

// Disagree
const OPTION_DISAGREE: &[Effect] = &[EVENT_CONSUME_EFFECT];

const LABELS_BASE: &[&str] = &[
    "[Agree] Gain 175 Gold. Become Cursed - Doubt.",
    "[Disagree] Nothing happens.",
];
const LABELS_A15: &[&str] = &[
    "[Agree] Gain 150 Gold. Become Cursed - Doubt.",
    "[Disagree] Nothing happens.",
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
        0 if ascension < 15 => &OPTION_AGREE_BASE,
        0 => &OPTION_AGREE_A15,
        1 => OPTION_DISAGREE,
        _ => unreachable!("the ssssserpent option out of range: {idx}"),
    });
}
