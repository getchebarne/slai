use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::make_entity_event_option;
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

static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option(
        "[Agree] Gain 175 Gold. Become Cursed - Doubt.",
        &OPTION_AGREE_BASE,
    ),
    make_entity_event_option("[Disagree] Nothing happens.", OPTION_DISAGREE),
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option(
        "[Agree] Gain 150 Gold. Become Cursed - Doubt.",
        &OPTION_AGREE_A15,
    ),
    make_entity_event_option("[Disagree] Nothing happens.", OPTION_DISAGREE),
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
