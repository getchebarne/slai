use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::DeckSelectKind;
use crate::types::EventName;

const HEAL: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 35 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::HealthGainPct { numer: 1, denom: 4 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const PURIFY: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 50 },
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

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const PURIFY_GATE: &[EventGate] = &[EventGate::GoldAtLeast(50), EventGate::HasPurgeableInDeck];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Heal (35 gold, +25% max HP)",
        effects: HEAL,
        gate: EventGate::GoldAtLeast(35),
    },
    EventOption {
        label: "Purification (50 gold, remove a card)",
        effects: PURIFY,
        gate: EventGate::All(PURIFY_GATE),
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static CLERIC: Entity = make_entity_event(EventName::Cleric, OPTIONS);
