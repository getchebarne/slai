use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

const POTION_EFFECT: Effect = Effect {
    kind: EffectKind::PotionAddRandom { limited: false },
    id_source: None,
    target: Target::Direct(None),
};

// Search; potions beyond free belt slots are lost (source lets you skip on a reward screen)
const OPTION_SEARCH_BASE: &[Effect] = &[
    POTION_EFFECT,
    POTION_EFFECT,
    POTION_EFFECT,
    EVENT_CONSUME_EFFECT,
];
const OPTION_SEARCH_A15: &[Effect] = &[POTION_EFFECT, POTION_EFFECT, EVENT_CONSUME_EFFECT];

// All options; the source game offers no way to decline
const OPTIONS_ALL_BASE: &[EventOption] = &[EventOption {
    label: "[Search] Obtain 3 random potions.",
    effects: OPTION_SEARCH_BASE,
    gate: EventGate::None,
}];
const OPTIONS_ALL_A15: &[EventOption] = &[EventOption {
    label: "[Search] Obtain 2 random potions.",
    effects: OPTION_SEARCH_A15,
    gate: EventGate::None,
}];

// Export event
static EVENT_THE_LAB_BASE: Entity = make_entity_event(EventName::TheLab, OPTIONS_ALL_BASE);
static EVENT_THE_LAB_A15: Entity = make_entity_event(EventName::TheLab, OPTIONS_ALL_A15);
pub fn spawn_event_the_lab(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_THE_LAB_BASE
    } else {
        EVENT_THE_LAB_A15
    }
}
