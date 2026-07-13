use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Drink: purge every removable curse at once (bottled/unpurgeable curses survive)
const OPTION_DRINK: &[Effect] = &[
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolDeckFilter::Curse,
            },
            selection_kind: SelectionKind::All,
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options; the event only spawns with a removable curse in the deck
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Drink] Remove all Curses from your deck.",
        effects: OPTION_DRINK,
        gate: EventGate::None,
    },
    EventOption {
        label: "[Leave] Nothing happens.",
        effects: OPTION_LEAVE,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_THE_DIVINE_FOUNTAIN: Entity =
    make_entity_event(EventName::TheDivineFountain, OPTIONS_ALL);
pub fn spawn_event_the_divine_fountain() -> Entity {
    EVENT_THE_DIVINE_FOUNTAIN
}
