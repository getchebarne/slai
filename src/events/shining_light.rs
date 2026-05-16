use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

const ENTER: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthLossPct { numer: 1, denom: 5 },
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

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Enter (lose 20% max HP, upgrade up to 2 random cards)",
        effects: ENTER,
        gate: EventGate::HasUpgradableInDeck,
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static SHINING_LIGHT: Entity = make_entity_event(EventName::ShiningLight, OPTIONS);
