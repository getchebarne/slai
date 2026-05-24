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

// Pinned sub-modes; StS rolls adjustment vs cleanup variants per spawn

const ADJUSTMENT: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 40 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardUpgradeRandomInDeck { count: 2 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const CLEAN_UP: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 60 },
        id_source: None,
        target: Target::Direct(None),
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

const FULL_SERVICE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 90 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::DeckSelectStart {
            kind: DeckSelectKind::Remove,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardUpgradeRandomInDeck { count: 1 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

// HP_LOSS bypasses block; HealthLoss already does that
const PUNCH: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthLoss { amount: 3 },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_END_EFFECT,
];

const ADJUST_GATE: &[EventGate] = &[EventGate::GoldAtLeast(40), EventGate::HasUpgradableInDeck];
const CLEAN_GATE: &[EventGate] = &[EventGate::GoldAtLeast(60), EventGate::HasPurgeableInDeck];
const FULL_GATE: &[EventGate] = &[EventGate::GoldAtLeast(90), EventGate::HasPurgeableInDeck];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Adjustment (40 gold, upgrade 2 random cards)",
        effects: ADJUSTMENT,
        gate: EventGate::All(ADJUST_GATE),
    },
    EventOption {
        label: "Clean Up (60 gold, remove a card)",
        effects: CLEAN_UP,
        gate: EventGate::All(CLEAN_GATE),
    },
    EventOption {
        label: "Full Service (90 gold, remove a card + upgrade 1 random)",
        effects: FULL_SERVICE,
        gate: EventGate::All(FULL_GATE),
    },
    EventOption {
        label: "Punch (lose 3 HP)",
        effects: PUNCH,
        gate: EventGate::None,
    },
];

pub static DESIGNER: Entity = make_entity_event(EventName::Designer, OPTIONS);
