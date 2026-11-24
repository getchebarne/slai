use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::ReadAt;
use crate::effect::Rounding;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_CARD_PILE_EVENT_ROLL_CLEAR;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::DeltaSign;

// Read: 20 unique rolled Cards wait in the event's roll; the selection keeps one, with no skip
const OPTION_READ: &[Effect] = &[
    EFFECT_EVENT_CONSUME,
    Effect {
        kind: EffectKind::LibraryCardsRoll,
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardAdopt { landing: false },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileEventRoll,
            filters: &[],
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EFFECT_CARD_PILE_EVENT_ROLL_CLEAR,
];

// Sleep: heal a third (A15+: a fifth) of max HP
const fn sleep(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Relative {
                    numerator,
                    denominator,
                    rounding: Rounding::HalfUp,
                    read_at: ReadAt::EventOpen,
                },
            },
            id_source: None,
            target: TARGET_CHARACTER,
        },
        EFFECT_EVENT_CONSUME,
    ]
}

// Sleep: heal a third of max HP
const OPTION_SLEEP_BASE: [Effect; 2] = sleep(33, 100);

// Sleep at A15+: only a fifth
const OPTION_SLEEP_A15: [Effect; 2] = sleep(20, 100);

static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_READ),
    make_event_option_template(&OPTION_SLEEP_BASE),
];
static EOTS_A15: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_READ),
    make_event_option_template(&OPTION_SLEEP_A15),
];

pub fn catalog(ascension: u8) -> &'static [EventOptionTemplate] {
    if ascension < 15 { EOTS_BASE } else { EOTS_A15 }
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
