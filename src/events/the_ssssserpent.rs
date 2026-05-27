use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::GoldDeltaKind;
use crate::effect::GoldDeltaSign;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::EventName;

// Agree
const fn agree(gold: u16) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: GoldDeltaSign::Gain,
                kind: GoldDeltaKind::Fixed(gold),
            },
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
static OPTION_AGREE_BASE: [Effect; 3] = agree(175);
static OPTION_AGREE_A15: [Effect; 3] = agree(150); // -25 gold gain

// Disagree
const OPTION_DISAGREE: &[Effect] = &[EVENT_END_EFFECT];

// All options
const fn options(agree_effects: &'static [Effect], agree_label: &'static str) -> [EventOption; 2] {
    [
        EventOption {
            label: agree_label,
            effects: agree_effects,
            gate: EventGate::None,
        },
        EventOption {
            label: "[Disagree] Nothing happens.",
            effects: OPTION_DISAGREE,
            gate: EventGate::None,
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 2] =
    options(&OPTION_AGREE_BASE, "[Agree] Gain 175 Gold. Become Cursed - Doubt.");
static OPTIONS_ALL_A15: [EventOption; 2] =
    options(&OPTION_AGREE_A15, "[Agree] Gain 150 Gold. Become Cursed - Doubt.");

// Export event
static EVENT_THE_SSSSSERPENT_BASE: Entity =
    make_entity_event(EventName::TheSsssserpent, &OPTIONS_ALL_BASE);
static EVENT_THE_SSSSSERPENT_A15: Entity =
    make_entity_event(EventName::TheSsssserpent, &OPTIONS_ALL_A15);
pub fn spawn_event_the_ssssserpent(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_THE_SSSSSERPENT_BASE
    } else {
        EVENT_THE_SSSSSERPENT_A15
    }
}
