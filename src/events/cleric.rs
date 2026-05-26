use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::CandidatePool;
use crate::effect::SelectionKind;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// A15+: purify cost 50 → 75

const HEAL: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 35 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::HealthDelta {
            sign: HealthDeltaSign::Gain,
            amount: HealthDeltaAmount::Pct { numer: 1, denom: 4 },
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const fn purify(cost: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldLoss { amount: cost },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardPurge,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck { filter: CandidatePoolDeckFilter::Purgeable },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
        EVENT_END_EFFECT,
    ]
}

static PURIFY_BASE: [Effect; 3] = purify(50);
static PURIFY_A15: [Effect; 3] = purify(75);

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const PURIFY_GATE_BASE: &[EventGate] = &[EventGate::GoldAtLeast(50), EventGate::HasPurgeableInDeck];
const PURIFY_GATE_A15: &[EventGate] = &[EventGate::GoldAtLeast(75), EventGate::HasPurgeableInDeck];

const fn options(
    purify_effects: &'static [Effect],
    purify_label: &'static str,
    purify_gate: &'static [EventGate],
) -> [EventOption; 3] {
    [
        EventOption {
            label: "Heal (35 gold, +25% max HP)",
            effects: HEAL,
            gate: EventGate::GoldAtLeast(35),
        },
        EventOption {
            label: purify_label,
            effects: purify_effects,
            gate: EventGate::All(purify_gate),
        },
        EventOption {
            label: "Leave",
            effects: LEAVE,
            gate: EventGate::None,
        },
    ]
}

static OPTIONS_BASE: [EventOption; 3] = options(
    &PURIFY_BASE,
    "Purification (50 gold, remove a card)",
    PURIFY_GATE_BASE,
);
static OPTIONS_A15: [EventOption; 3] = options(
    &PURIFY_A15,
    "Purification (75 gold, remove a card)",
    PURIFY_GATE_A15,
);

pub static CLERIC_BASE: Entity = make_entity_event(EventName::Cleric, &OPTIONS_BASE);
pub static CLERIC_A15: Entity = make_entity_event(EventName::Cleric, &OPTIONS_A15);

pub fn spawn_event_cleric(ascension: u8) -> Entity {
    if ascension < 15 {
        CLERIC_BASE
    } else {
        CLERIC_A15
    }
}
