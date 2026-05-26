use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::Target;
use crate::effect::EffectKind;
use crate::effect::CandidatePool;
use crate::effect::SelectionKind;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

const FORGET: &[Effect] = &[
    Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck { filter: CandidatePoolDeckFilter::Purgeable },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
    EVENT_END_EFFECT,
];

const CHANGE: &[Effect] = &[
    Effect {
            kind: EffectKind::CardTransform,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck { filter: CandidatePoolDeckFilter::Transformable },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
    EVENT_END_EFFECT,
];

const GROW: &[Effect] = &[
    Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck { filter: CandidatePoolDeckFilter::Upgradeable },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
    EVENT_END_EFFECT,
];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Forget (remove a card)",
        effects: FORGET,
        gate: EventGate::HasPurgeableInDeck,
    },
    EventOption {
        label: "Change (transform a card)",
        effects: CHANGE,
        gate: EventGate::HasNonBasicNonCurseInDeck,
    },
    EventOption {
        label: "Grow (upgrade a card)",
        effects: GROW,
        gate: EventGate::HasUpgradableInDeck,
    },
];

pub static LIVING_WALL: Entity = make_entity_event(EventName::LivingWall, OPTIONS);
