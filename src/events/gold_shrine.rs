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

// A15+: pray gold 100 → 50

const fn pray(amount: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldGain { amount },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_END_EFFECT,
    ]
}

static PRAY_BASE: [Effect; 2] = pray(100);
static PRAY_A15: [Effect; 2] = pray(50);

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

const fn options(pray_effects: &'static [Effect], pray_label: &'static str) -> [EventOption; 3] {
    [
        EventOption {
            label: pray_label,
            effects: pray_effects,
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
    ]
}

static OPTIONS_BASE: [EventOption; 3] = options(&PRAY_BASE, "Pray (+100 gold)");
static OPTIONS_A15: [EventOption; 3] = options(&PRAY_A15, "Pray (+50 gold)");

pub static GOLD_SHRINE_BASE: Entity = make_entity_event(EventName::GoldShrine, &OPTIONS_BASE);
pub static GOLD_SHRINE_A15: Entity = make_entity_event(EventName::GoldShrine, &OPTIONS_A15);

pub fn spawn_event_gold_shrine(ascension: u8) -> Entity {
    if ascension < 15 {
        GOLD_SHRINE_BASE
    } else {
        GOLD_SHRINE_A15
    }
}
