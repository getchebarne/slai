use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::EventName;

const PRAY: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldGain { amount: 100 },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const DESECRATE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldGain { amount: 275 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Regret,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Pray (+100 gold)",
        effects: PRAY,
        gate: EventGate::None,
    },
    EventOption {
        label: "Desecrate (+275 gold, +Regret curse)",
        effects: DESECRATE,
        gate: EventGate::None,
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static GOLD_SHRINE: Entity = make_entity_event(EventName::GoldShrine, OPTIONS);
