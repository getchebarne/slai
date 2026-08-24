use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EFFECT_DECK_PURGE_PICK_1;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::deck_has_purgeable;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Simplicity: every un-upgraded Strike and Defend upgrades
const OPTION_SIMPLICITY: &[Effect] = &[
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::StarterUpgradeable,
            selection_kind: SelectionKind::All,
        },
    },
    EFFECT_EVENT_CONSUME,
];

// Elegance: purge a Card
const OPTION_ELEGANCE: &[Effect] = &[EFFECT_DECK_PURGE_PICK_1, EFFECT_EVENT_CONSUME];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_ELEGANCE),
    make_event_option_template(OPTION_SIMPLICITY),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        _ => true,
    }
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}
