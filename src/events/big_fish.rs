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
use crate::types::CardName;
use crate::types::EventName;

const BANANA: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthGainPct { numer: 1, denom: 3 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const DONUT: &[Effect] = &[
    Effect {
        kind: EffectKind::MaxHealthGain { amount: 5 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_END_EFFECT,
];

const BOX: &[Effect] = &[
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Regret,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::RelicGrantRandom { tier: None },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Banana (heal 1/3 max HP)",
        effects: BANANA,
        gate: EventGate::None,
    },
    EventOption {
        label: "Donut (+5 max HP)",
        effects: DONUT,
        gate: EventGate::None,
    },
    EventOption {
        label: "Box (curse + random relic)",
        effects: BOX,
        gate: EventGate::None,
    },
];

pub static BIG_FISH: Entity = make_entity_event(EventName::BigFish, OPTIONS);
