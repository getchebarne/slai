use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::GoldDeltaKind;
use crate::types::DeltaSign;
use crate::effect::HealthDeltaAmount;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Heal
const OPTION_HEAL: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            kind: GoldDeltaKind::Fixed(35),
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: HealthDeltaAmount::Relative {
                numerator: 1,
                denominator: 4,
            },
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_END_EFFECT,
];

// Purify
const fn purify(cost: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                kind: GoldDeltaKind::Fixed(cost),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck {
                    filter: CandidatePoolDeckFilter::Purgeable,
                },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
        EVENT_END_EFFECT,
    ]
}
static OPTION_PURIFY_BASE: [Effect; 3] = purify(50);
static OPTION_PURIFY_A15: [Effect; 3] = purify(75); // +25 gold cost
const OPTION_PURIFY_GATE_BASE: &[EventGate] =
    &[EventGate::GoldAtLeast(50), EventGate::HasPurgeableInDeck];
const OPTION_PURIFY_GATE_A15: &[EventGate] =
    &[EventGate::GoldAtLeast(75), EventGate::HasPurgeableInDeck];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_END_EFFECT];

// All options
const fn options(
    purify_effects: &'static [Effect],
    purify_label: &'static str,
    purify_gate: &'static [EventGate],
) -> [EventOption; 3] {
    [
        EventOption {
            label: "[Heal] Pay 35 Gold. Heal 25% of your max HP.",
            effects: OPTION_HEAL,
            gate: EventGate::GoldAtLeast(35),
        },
        EventOption {
            label: purify_label,
            effects: purify_effects,
            gate: EventGate::All(purify_gate),
        },
        EventOption {
            label: "[Leave] Nothing happens.",
            effects: OPTION_LEAVE,
            gate: EventGate::None,
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 3] = options(
    &OPTION_PURIFY_BASE,
    "[Purify] Pay 50 Gold. Remove a card from your deck.",
    OPTION_PURIFY_GATE_BASE,
);
static OPTIONS_ALL_A15: [EventOption; 3] = options(
    &OPTION_PURIFY_A15,
    "[Purify] Pay 75 Gold. Remove a card from your deck.",
    OPTION_PURIFY_GATE_A15,
);

// Export event
static EVENT_THE_CLERIC_BASE: Entity = make_entity_event(EventName::TheCleric, &OPTIONS_ALL_BASE);
static EVENT_THE_CLERIC_A15: Entity = make_entity_event(EventName::TheCleric, &OPTIONS_ALL_A15);
pub fn spawn_event_the_cleric(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_THE_CLERIC_BASE
    } else {
        EVENT_THE_CLERIC_A15
    }
}
