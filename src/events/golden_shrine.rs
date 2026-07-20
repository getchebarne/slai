use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
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
static OPTION_PRAY_BASE: [Effect; 2] = pray(100);
static OPTION_PRAY_A15: [Effect; 2] = pray(50);

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

const LABELS_BASE: &[&str] = &[
    "[Pray] Gain 100 Gold.",
    "[Desecrate] Gain 275 Gold. Become Cursed - Regret.",
    "[Leave] Nothing happens.",
];
const LABELS_A15: &[&str] = &[
    "[Pray] Gain 50 Gold.",
    "[Desecrate] Gain 275 Gold. Become Cursed - Regret.",
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
        0 if ascension < 15 => &OPTION_PRAY_BASE,
        0 => &OPTION_PRAY_A15,
        1 => OPTION_DESECRATE,
        2 => OPTION_LEAVE,
        _ => unreachable!("golden shrine option out of range: {idx}"),
    });
}
