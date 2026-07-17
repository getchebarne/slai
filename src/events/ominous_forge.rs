use crate::effect::CandidatePool;
use crate::effect::CandidatePoolCardFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::EventName;
use crate::types::RelicName;

// Forge
const OPTION_FORGE: &[Effect] = &[
    Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck {
                filter: CandidatePoolCardFilter::Upgradeable,
            },
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    EVENT_CONSUME_EFFECT,
];

// Rummage
const OPTION_RUMMAGE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::WarpedTongs,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Pain,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Forge] Upgrade a card.",
        effects: OPTION_FORGE,
        gate: EventGate::HasUpgradableInDeck,
    },
    EventOption {
        label: "[Rummage] Obtain Warped Tongs. Become Cursed - Pain.",
        effects: OPTION_RUMMAGE,
        gate: EventGate::None,
    },
    EventOption {
        label: "[Leave] Nothing happens.",
        effects: OPTION_LEAVE,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_OMINOUS_FORGE: Entity = make_entity_event(EventName::OminousForge, OPTIONS_ALL);
pub fn spawn_event_ominous_forge() -> Entity {
    EVENT_OMINOUS_FORGE
}
