use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::make_entity_event_option;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;

// Accept: lose half your max HP (rounded up), gain Apparitions
const fn accept(count: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: Amount::RelativeCeil {
                    numerator: 1,
                    denominator: 2,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        Effect {
            kind: EffectKind::CardAdd {
                card_name: CardName::Apparition,
                pile: CardPile::Deck,
                count,
                upgraded: false,
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
const OPTION_ACCEPT_BASE: [Effect; 3] = accept(5);
const OPTION_ACCEPT_A15: [Effect; 3] = accept(3);

static OPTIONS_BASE: &[Entity] = &[
    make_entity_event_option(
        "[Accept] Lose half your Max HP. Obtain 5 Apparitions.",
        &OPTION_ACCEPT_BASE,
    ),
    OPTION_LEAVE,
];
static OPTIONS_A15: &[Entity] = &[
    make_entity_event_option(
        "[Accept] Lose half your Max HP. Obtain 3 Apparitions.",
        &OPTION_ACCEPT_A15,
    ),
    OPTION_LEAVE,
];

pub fn options(ascension: u8) -> &'static [Entity] {
    if ascension < 15 {
        OPTIONS_BASE
    } else {
        OPTIONS_A15
    }
}
