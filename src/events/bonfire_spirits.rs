use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;

// Offer: pick a Card to purge. An empty purgeable pool auto-resolves to nothing
const OPTION_OFFER: &[Effect] = &[
    Effect {
        kind: EffectKind::BonfireOffer,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Purgeable,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[make_event_option_template(OPTION_OFFER)];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
