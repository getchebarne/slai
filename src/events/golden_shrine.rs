use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::EventName;

// Pray
const fn pray(amount: u16) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(amount),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_PRAY_BASE: [Effect; 2] = pray(100);
static OPTION_PRAY_A15: [Effect; 2] = pray(50); // -50 gold gain

// Desecrate
const OPTION_DESECRATE: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Absolute(275),
        },
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
    EVENT_CONSUME_EFFECT,
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options
const fn options(pray_effects: &'static [Effect], pray_label: &'static str) -> [EventOption; 3] {
    [
        EventOption {
            label: pray_label,
            effects: pray_effects,
            gate: EventGate::None,
        },
        EventOption {
            label: "[Desecrate] Gain 275 Gold. Become Cursed - Regret.",
            effects: OPTION_DESECRATE,
            gate: EventGate::None,
        },
        EventOption {
            label: "[Leave] Nothing happens.",
            effects: OPTION_LEAVE,
            gate: EventGate::None,
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 3] = options(&OPTION_PRAY_BASE, "[Pray] Gain 100 Gold.");
static OPTIONS_ALL_A15: [EventOption; 3] = options(&OPTION_PRAY_A15, "[Pray] Gain 50 Gold.");

// Export event
static EVENT_GOLDEN_SHRINE_BASE: Entity =
    make_entity_event(EventName::GoldenShrine, &OPTIONS_ALL_BASE);
static EVENT_GOLDEN_SHRINE_A15: Entity =
    make_entity_event(EventName::GoldenShrine, &OPTIONS_ALL_A15);
pub fn spawn_event_golden_shrine(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_GOLDEN_SHRINE_BASE
    } else {
        EVENT_GOLDEN_SHRINE_A15
    }
}
