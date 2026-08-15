use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EFFECT_DECK_PURGE_PICK;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::deck_has_purgeable;
use crate::events::make_entity_event_option;
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
    EVENT_CONSUME_EFFECT,
];

const OPTION_ELEGANCE: &[Effect] = &[EFFECT_DECK_PURGE_PICK, EVENT_CONSUME_EFFECT];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Elegance] Remove a card from your deck.", OPTION_ELEGANCE),
    make_entity_event_option(
        "[Simplicity] Upgrade all your Strikes and Defends.",
        OPTION_SIMPLICITY,
    ),
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        0 => deck_has_purgeable(state),
        _ => true,
    }
}
