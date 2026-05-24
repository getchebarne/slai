use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::DeckSelectKind;
use crate::types::EventName;

const PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthLoss { amount: 7 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::DeckSelectStart {
            kind: DeckSelectKind::Remove,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

// StS uses rand(50,80); fixed at midpoint pending range-aware effect kinds
const ATTACK: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldGain { amount: 65 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Pray (lose 7 HP, remove a card)",
        effects: PRAY,
        gate: EventGate::HasPurgeableInDeck,
    },
    EventOption {
        label: "Attack (+65 gold)",
        effects: ATTACK,
        gate: EventGate::HasDamageCardInDeck { min_base: 10 },
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static GOLDEN_WING: Entity = make_entity_event(EventName::GoldenWing, OPTIONS);
