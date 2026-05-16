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

const AGREE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldGain { amount: 175 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Doubt,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const DISAGREE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Agree (+175 gold, +Doubt curse)",
        effects: AGREE,
        gate: EventGate::None,
    },
    EventOption {
        label: "Disagree",
        effects: DISAGREE,
        gate: EventGate::None,
    },
];

pub static SSSSERPENT: Entity = make_entity_event(EventName::Sssserpent, OPTIONS);
