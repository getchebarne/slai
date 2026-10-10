use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::types::CardPile;

// Pray; consume first, so the pick lands over the spent event
const OPTION_PRAY: &[Effect] = &[
    EFFECT_EVENT_CONSUME,
    Effect {
        kind: EffectKind::CardDuplicate {
            card_pile: CardPile::Deck,
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::CardPileDeck,
            filters: &[],
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
];

// Leave
pub static EOTS_BASE: &[EventOptionTemplate] =
    &[make_event_option_template(OPTION_PRAY), EOT_LEAVE];

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn spawn(state: &mut GameState) -> Vec<usize> {
    bake_options(state, catalog(state.ascension))
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
