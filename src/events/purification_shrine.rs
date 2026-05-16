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

const PRAY: &[Effect] = &[
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

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Pray (remove a card)",
        effects: PRAY,
        gate: EventGate::HasPurgeableInDeck,
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static PURIFICATION_SHRINE: Entity =
    make_entity_event(EventName::PurificationShrine, OPTIONS);
