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

const PRAY: &[Effect] = &[
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

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Pray (transform a card)",
        effects: PRAY,
        gate: EventGate::HasNonBasicNonCurseInDeck,
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static TRANSMOGRIFIER: Entity = make_entity_event(EventName::Transmogrifier, OPTIONS);
