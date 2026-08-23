use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::opt;
use crate::types::CardName;
use crate::types::CardPile;
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
            kind: EffectKind::CardAdd {
                card_name: CardName::Doubt,
                pile: CardPile::Deck,
                count: 1,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_AGREE_BASE: [Effect; 3] = agree(175);
const OPTION_AGREE_A15: [Effect; 3] = agree(150);

// Disagree
const OPTION_DISAGREE: &[Effect] = &[EVENT_CONSUME_EFFECT];

static OPTIONS_BASE: &[&[Effect]] = &[opt(&OPTION_AGREE_BASE), opt(OPTION_DISAGREE)];
static OPTIONS_A15: &[&[Effect]] = &[opt(&OPTION_AGREE_A15), opt(OPTION_DISAGREE)];

pub fn options(ascension: u8) -> &'static [&'static [Effect]] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
