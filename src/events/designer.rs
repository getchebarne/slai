use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

const fn adjustment(cost: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldLoss { amount: cost },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck {
                    filter: CandidatePoolDeckFilter::Upgradeable,
                },
                selection_kind: SelectionKind::Random { count: 2 },
            },
        },
        EVENT_END_EFFECT,
    ]
}

const fn clean_up(cost: u16) -> [Effect; 3] {
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
                candidate_pool: CandidatePool::Deck {
                    filter: CandidatePoolDeckFilter::Purgeable,
                },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
        EVENT_END_EFFECT,
    ]
}

const fn full_service(cost: u16) -> [Effect; 4] {
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
                candidate_pool: CandidatePool::Deck {
                    filter: CandidatePoolDeckFilter::Purgeable,
                },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck {
                    filter: CandidatePoolDeckFilter::Upgradeable,
                },
                selection_kind: SelectionKind::Random { count: 1 },
            },
        },
        EVENT_END_EFFECT,
    ]
}

// HP_LOSS bypasses block; HealthLoss already does that
const fn punch(dmg: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: HealthDeltaSign::Loss,
                amount: HealthDeltaAmount::Flat(dmg),
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        EVENT_END_EFFECT,
    ]
}

static ADJUSTMENT_BASE: [Effect; 3] = adjustment(40);
static ADJUSTMENT_A15: [Effect; 3] = adjustment(50);
static CLEAN_UP_BASE: [Effect; 3] = clean_up(60);
static CLEAN_UP_A15: [Effect; 3] = clean_up(75);
static FULL_SERVICE_BASE: [Effect; 4] = full_service(90);
static FULL_SERVICE_A15: [Effect; 4] = full_service(110);
static PUNCH_BASE: [Effect; 2] = punch(3);
static PUNCH_A15: [Effect; 2] = punch(5);

const ADJUST_GATE_BASE: &[EventGate] =
    &[EventGate::GoldAtLeast(40), EventGate::HasUpgradableInDeck];
const ADJUST_GATE_A15: &[EventGate] = &[EventGate::GoldAtLeast(50), EventGate::HasUpgradableInDeck];
const CLEAN_GATE_BASE: &[EventGate] = &[EventGate::GoldAtLeast(60), EventGate::HasPurgeableInDeck];
const CLEAN_GATE_A15: &[EventGate] = &[EventGate::GoldAtLeast(75), EventGate::HasPurgeableInDeck];
const FULL_GATE_BASE: &[EventGate] = &[EventGate::GoldAtLeast(90), EventGate::HasPurgeableInDeck];
const FULL_GATE_A15: &[EventGate] = &[EventGate::GoldAtLeast(110), EventGate::HasPurgeableInDeck];

const OPTIONS_BASE: &[EventOption] = &[
    EventOption {
        label: "Adjustment (40 gold, upgrade 2 random cards)",
        effects: &ADJUSTMENT_BASE,
        gate: EventGate::All(ADJUST_GATE_BASE),
    },
    EventOption {
        label: "Clean Up (60 gold, remove a card)",
        effects: &CLEAN_UP_BASE,
        gate: EventGate::All(CLEAN_GATE_BASE),
    },
    EventOption {
        label: "Full Service (90 gold, remove a card + upgrade 1)",
        effects: &FULL_SERVICE_BASE,
        gate: EventGate::All(FULL_GATE_BASE),
    },
    EventOption {
        label: "Punch (lose 3 HP)",
        effects: &PUNCH_BASE,
        gate: EventGate::None,
    },
];

const OPTIONS_A15: &[EventOption] = &[
    EventOption {
        label: "Adjustment (50 gold, upgrade 2 random cards)",
        effects: &ADJUSTMENT_A15,
        gate: EventGate::All(ADJUST_GATE_A15),
    },
    EventOption {
        label: "Clean Up (75 gold, remove a card)",
        effects: &CLEAN_UP_A15,
        gate: EventGate::All(CLEAN_GATE_A15),
    },
    EventOption {
        label: "Full Service (110 gold, remove a card + upgrade 1)",
        effects: &FULL_SERVICE_A15,
        gate: EventGate::All(FULL_GATE_A15),
    },
    EventOption {
        label: "Punch (lose 5 HP)",
        effects: &PUNCH_A15,
        gate: EventGate::None,
    },
];

pub static DESIGNER_BASE: Entity = make_entity_event(EventName::Designer, OPTIONS_BASE);
pub static DESIGNER_A15: Entity = make_entity_event(EventName::Designer, OPTIONS_A15);

pub fn spawn_event_designer(ascension: u8) -> Entity {
    if ascension < 15 {
        DESIGNER_BASE
    } else {
        DESIGNER_A15
    }
}
