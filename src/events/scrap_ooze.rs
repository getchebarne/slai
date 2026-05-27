use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// Reach in (base); state 7 self-loops on miss (no advance)
const fn reach(dmg: u16, chance: u8, advance_on_miss: bool) -> [Effect; 1] {
    [Effect {
        kind: EffectKind::ScrapOozeReach {
            dmg,
            chance,
            advance_on_miss,
        },
        id_source: None,
        target: Target::Direct(None),
    }]
}
static OPTION_REACH_BASE_0: [Effect; 1] = reach(3, 25, true);
static OPTION_REACH_BASE_1: [Effect; 1] = reach(4, 35, true);
static OPTION_REACH_BASE_2: [Effect; 1] = reach(5, 45, true);
static OPTION_REACH_BASE_3: [Effect; 1] = reach(6, 55, true);
static OPTION_REACH_BASE_4: [Effect; 1] = reach(7, 65, true);
static OPTION_REACH_BASE_5: [Effect; 1] = reach(8, 75, true);
static OPTION_REACH_BASE_6: [Effect; 1] = reach(9, 85, true);
static OPTION_REACH_BASE_7: [Effect; 1] = reach(10, 95, false);

// Base damage 3 -> 5
static OPTION_REACH_A15_0: [Effect; 1] = reach(5, 25, true);
static OPTION_REACH_A15_1: [Effect; 1] = reach(6, 35, true);
static OPTION_REACH_A15_2: [Effect; 1] = reach(7, 45, true);
static OPTION_REACH_A15_3: [Effect; 1] = reach(8, 55, true);
static OPTION_REACH_A15_4: [Effect; 1] = reach(9, 65, true);
static OPTION_REACH_A15_5: [Effect; 1] = reach(10, 75, true);
static OPTION_REACH_A15_6: [Effect; 1] = reach(11, 85, true);
static OPTION_REACH_A15_7: [Effect; 1] = reach(12, 95, false);

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_END_EFFECT];

// All options
const OPTIONS_ALL_BASE: &[EventOption] = &[
    EventOption {
        label: "[Reach Inside] Lose 3 HP. 25% chance for a Relic.",
        effects: &OPTION_REACH_BASE_0,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "[Reach Inside] Lose 4 HP. 35% chance for a Relic.",
        effects: &OPTION_REACH_BASE_1,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "[Reach Inside] Lose 5 HP. 45% chance for a Relic.",
        effects: &OPTION_REACH_BASE_2,
        gate: EventGate::EventStateEq(2),
    },
    EventOption {
        label: "[Reach Inside] Lose 6 HP. 55% chance for a Relic.",
        effects: &OPTION_REACH_BASE_3,
        gate: EventGate::EventStateEq(3),
    },
    EventOption {
        label: "[Reach Inside] Lose 7 HP. 65% chance for a Relic.",
        effects: &OPTION_REACH_BASE_4,
        gate: EventGate::EventStateEq(4),
    },
    EventOption {
        label: "[Reach Inside] Lose 8 HP. 75% chance for a Relic.",
        effects: &OPTION_REACH_BASE_5,
        gate: EventGate::EventStateEq(5),
    },
    EventOption {
        label: "[Reach Inside] Lose 9 HP. 85% chance for a Relic.",
        effects: &OPTION_REACH_BASE_6,
        gate: EventGate::EventStateEq(6),
    },
    EventOption {
        label: "[Reach Inside] Lose 10 HP. 95% chance for a Relic.",
        effects: &OPTION_REACH_BASE_7,
        gate: EventGate::EventStateEq(7),
    },
    EventOption {
        label: "[Leave] Nothing happens.",
        effects: OPTION_LEAVE,
        gate: EventGate::None,
    },
];
const OPTIONS_ALL_A15: &[EventOption] = &[
    EventOption {
        label: "[Reach Inside] Lose 5 HP. 25% chance for a Relic.",
        effects: &OPTION_REACH_A15_0,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "[Reach Inside] Lose 6 HP. 35% chance for a Relic.",
        effects: &OPTION_REACH_A15_1,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "[Reach Inside] Lose 7 HP. 45% chance for a Relic.",
        effects: &OPTION_REACH_A15_2,
        gate: EventGate::EventStateEq(2),
    },
    EventOption {
        label: "[Reach Inside] Lose 8 HP. 55% chance for a Relic.",
        effects: &OPTION_REACH_A15_3,
        gate: EventGate::EventStateEq(3),
    },
    EventOption {
        label: "[Reach Inside] Lose 9 HP. 65% chance for a Relic.",
        effects: &OPTION_REACH_A15_4,
        gate: EventGate::EventStateEq(4),
    },
    EventOption {
        label: "[Reach Inside] Lose 10 HP. 75% chance for a Relic.",
        effects: &OPTION_REACH_A15_5,
        gate: EventGate::EventStateEq(5),
    },
    EventOption {
        label: "[Reach Inside] Lose 11 HP. 85% chance for a Relic.",
        effects: &OPTION_REACH_A15_6,
        gate: EventGate::EventStateEq(6),
    },
    EventOption {
        label: "[Reach Inside] Lose 12 HP. 95% chance for a Relic.",
        effects: &OPTION_REACH_A15_7,
        gate: EventGate::EventStateEq(7),
    },
    EventOption {
        label: "[Leave] Nothing happens.",
        effects: OPTION_LEAVE,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_SCRAP_OOZE_BASE: Entity = make_entity_event(EventName::ScrapOoze, OPTIONS_ALL_BASE);
static EVENT_SCRAP_OOZE_A15: Entity = make_entity_event(EventName::ScrapOoze, OPTIONS_ALL_A15);
pub fn spawn_event_scrap_ooze(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_SCRAP_OOZE_BASE
    } else {
        EVENT_SCRAP_OOZE_A15
    }
}
