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

// A15+: agree gold 175 → 150

const fn agree(gold: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldGain { amount: gold },
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
    ]
}

static AGREE_BASE: [Effect; 3] = agree(175);
static AGREE_A15: [Effect; 3] = agree(150);

const DISAGREE: &[Effect] = &[EVENT_END_EFFECT];

const fn options(agree_effects: &'static [Effect], agree_label: &'static str) -> [EventOption; 2] {
    [
        EventOption {
            label: agree_label,
            effects: agree_effects,
            gate: EventGate::None,
        },
        EventOption {
            label: "Disagree",
            effects: DISAGREE,
            gate: EventGate::None,
        },
    ]
}

static OPTIONS_BASE: [EventOption; 2] = options(&AGREE_BASE, "Agree (+175 gold, +Doubt curse)");
static OPTIONS_A15: [EventOption; 2] = options(&AGREE_A15, "Agree (+150 gold, +Doubt curse)");

pub static SSSSERPENT_BASE: Entity = make_entity_event(EventName::Sssserpent, &OPTIONS_BASE);
pub static SSSSERPENT_A15: Entity = make_entity_event(EventName::Sssserpent, &OPTIONS_A15);

pub fn spawn_event_sssserpent(ascension: u8) -> Entity {
    if ascension < 15 {
        SSSSERPENT_BASE
    } else {
        SSSSERPENT_A15
    }
}
