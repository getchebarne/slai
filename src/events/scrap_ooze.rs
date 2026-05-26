use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::HealthDeltaSign;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

// 8 escalating states (chance 25..=95, dmg+1/state); state 7 self-loops on miss. A15+ base dmg 3→5

const HIT: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantRandom,
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_END_EFFECT,
];
const ADVANCE: Effect = Effect {
    kind: EffectKind::EventAdvanceState { delta: 1 },
    id_source: None,
    target: Target::Direct(None),
};
const MISS: &[Effect] = &[ADVANCE];
const TERMINAL_MISS: &[Effect] = &[];

// Reach in (base)
const fn reach(dmg: u16, chance: u8, on_miss: &'static [Effect]) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: HealthDeltaSign::Loss,
                amount: HealthDeltaAmount::Absolute(dmg),
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        Effect {
            kind: EffectKind::RollD100Branch {
                chance,
                on_lt: HIT,
                on_ge: on_miss,
            },
            id_source: None,
            target: Target::Direct(None),
        },
    ]
}
static OPTION_REACH_BASE_0: [Effect; 2] = reach(3, 25, MISS);
static OPTION_REACH_BASE_1: [Effect; 2] = reach(4, 35, MISS);
static OPTION_REACH_BASE_2: [Effect; 2] = reach(5, 45, MISS);
static OPTION_REACH_BASE_3: [Effect; 2] = reach(6, 55, MISS);
static OPTION_REACH_BASE_4: [Effect; 2] = reach(7, 65, MISS);
static OPTION_REACH_BASE_5: [Effect; 2] = reach(8, 75, MISS);
static OPTION_REACH_BASE_6: [Effect; 2] = reach(9, 85, MISS);
static OPTION_REACH_BASE_7: [Effect; 2] = reach(10, 95, TERMINAL_MISS);
// Reach in (A15+); base dmg 3→5, +1 per state
static OPTION_REACH_A15_0: [Effect; 2] = reach(5, 25, MISS);
static OPTION_REACH_A15_1: [Effect; 2] = reach(6, 35, MISS);
static OPTION_REACH_A15_2: [Effect; 2] = reach(7, 45, MISS);
static OPTION_REACH_A15_3: [Effect; 2] = reach(8, 55, MISS);
static OPTION_REACH_A15_4: [Effect; 2] = reach(9, 65, MISS);
static OPTION_REACH_A15_5: [Effect; 2] = reach(10, 75, MISS);
static OPTION_REACH_A15_6: [Effect; 2] = reach(11, 85, MISS);
static OPTION_REACH_A15_7: [Effect; 2] = reach(12, 95, TERMINAL_MISS);
// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_END_EFFECT];

// All options
const OPTIONS_ALL_BASE: &[EventOption] = &[
    EventOption {
        label: "Reach in (lose 3 HP, 25% relic)",
        effects: &OPTION_REACH_BASE_0,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "Reach in (lose 4 HP, 35% relic)",
        effects: &OPTION_REACH_BASE_1,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "Reach in (lose 5 HP, 45% relic)",
        effects: &OPTION_REACH_BASE_2,
        gate: EventGate::EventStateEq(2),
    },
    EventOption {
        label: "Reach in (lose 6 HP, 55% relic)",
        effects: &OPTION_REACH_BASE_3,
        gate: EventGate::EventStateEq(3),
    },
    EventOption {
        label: "Reach in (lose 7 HP, 65% relic)",
        effects: &OPTION_REACH_BASE_4,
        gate: EventGate::EventStateEq(4),
    },
    EventOption {
        label: "Reach in (lose 8 HP, 75% relic)",
        effects: &OPTION_REACH_BASE_5,
        gate: EventGate::EventStateEq(5),
    },
    EventOption {
        label: "Reach in (lose 9 HP, 85% relic)",
        effects: &OPTION_REACH_BASE_6,
        gate: EventGate::EventStateEq(6),
    },
    EventOption {
        label: "Reach in (lose 10 HP, 95% relic)",
        effects: &OPTION_REACH_BASE_7,
        gate: EventGate::EventStateEq(7),
    },
    EventOption {
        label: "Leave",
        effects: OPTION_LEAVE,
        gate: EventGate::None,
    },
];
const OPTIONS_ALL_A15: &[EventOption] = &[
    EventOption {
        label: "Reach in (lose 5 HP, 25% relic)",
        effects: &OPTION_REACH_A15_0,
        gate: EventGate::EventStateEq(0),
    },
    EventOption {
        label: "Reach in (lose 6 HP, 35% relic)",
        effects: &OPTION_REACH_A15_1,
        gate: EventGate::EventStateEq(1),
    },
    EventOption {
        label: "Reach in (lose 7 HP, 45% relic)",
        effects: &OPTION_REACH_A15_2,
        gate: EventGate::EventStateEq(2),
    },
    EventOption {
        label: "Reach in (lose 8 HP, 55% relic)",
        effects: &OPTION_REACH_A15_3,
        gate: EventGate::EventStateEq(3),
    },
    EventOption {
        label: "Reach in (lose 9 HP, 65% relic)",
        effects: &OPTION_REACH_A15_4,
        gate: EventGate::EventStateEq(4),
    },
    EventOption {
        label: "Reach in (lose 10 HP, 75% relic)",
        effects: &OPTION_REACH_A15_5,
        gate: EventGate::EventStateEq(5),
    },
    EventOption {
        label: "Reach in (lose 11 HP, 85% relic)",
        effects: &OPTION_REACH_A15_6,
        gate: EventGate::EventStateEq(6),
    },
    EventOption {
        label: "Reach in (lose 12 HP, 95% relic)",
        effects: &OPTION_REACH_A15_7,
        gate: EventGate::EventStateEq(7),
    },
    EventOption {
        label: "Leave",
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
