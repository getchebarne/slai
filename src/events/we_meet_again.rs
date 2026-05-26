use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Gold-only subset; StS also offers a rolled potion or card

const GIVE_GOLD: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldLoss { amount: 100 },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::RelicGrantRandom,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];

const REFUSE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Give 100 gold (gain random relic)",
        effects: GIVE_GOLD,
        gate: EventGate::GoldAtLeast(100),
    },
    EventOption {
        label: "Refuse",
        effects: REFUSE,
        gate: EventGate::None,
    },
];

pub static WE_MEET_AGAIN: Entity = make_entity_event(EventName::WeMeetAgain, OPTIONS);
