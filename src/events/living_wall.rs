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

const FORGET: &[Effect] = &[
    Effect {
        kind: EffectKind::DeckSelectStart {
            kind: DeckSelectKind::Remove,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const CHANGE: &[Effect] = &[
    Effect {
        kind: EffectKind::DeckSelectStart {
            kind: DeckSelectKind::TransformOne,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const GROW: &[Effect] = &[
    Effect {
        kind: EffectKind::DeckSelectStart {
            kind: DeckSelectKind::UpgradeAny,
        },
        id_source: None,
        target: Target::Direct(None),
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
