use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
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
        EFFECT_EVENT_CONSUME,
    ]
}

// Accept: half max HP for 5 Apparitions
const OPTION_ACCEPT_BASE: [Effect; 3] = accept(5);

// Accept at A15+: only 3 Apparitions
const OPTION_ACCEPT_A15: [Effect; 3] = accept(3);

static EOTS_BASE: &[EventOptionTemplate] =
    &[make_event_option_template(&OPTION_ACCEPT_BASE), EOT_LEAVE];
static EOTS_A15: &[EventOptionTemplate] =
    &[make_event_option_template(&OPTION_ACCEPT_A15), EOT_LEAVE];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
