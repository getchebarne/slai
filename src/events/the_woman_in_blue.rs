use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::DeltaSign;
use crate::types::EventName;

const fn buy(cost: u16, count: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(cost),
            },
            id_source: None,
            target: Target::Direct(None),
        },
        // Rolled potions land on the reward screen, where the belt is interactive
        // (discard-to-swap), matching the source's combatRewardScreen
        Effect {
            kind: EffectKind::RewardRollPotions { count },
            id_source: None,
            target: Target::Direct(None),
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_BUY_1: [Effect; 3] = buy(20, 1);
static OPTION_BUY_2: [Effect; 3] = buy(30, 2);
static OPTION_BUY_3: [Effect; 3] = buy(40, 3);

// Leave: free below A15; costs ceil(5% max HP) at A15+
const OPTION_LEAVE_BASE: &[Effect] = &[EVENT_CONSUME_EFFECT];
const OPTION_LEAVE_A15: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::RelativeCeil {
                numerator: 1,
                denominator: 20,
            },
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    EVENT_CONSUME_EFFECT,
];

// All options; the event only spawns with >= 50 gold, which covers every price
const fn options(leave_effects: &'static [Effect], leave_label: &'static str) -> [EventOption; 4] {
    [
        EventOption {
            label: "[Buy 1 Potion] Lose 20 Gold.",
            effects: &OPTION_BUY_1,
            gate: EventGate::None,
        },
        EventOption {
            label: "[Buy 2 Potions] Lose 30 Gold.",
            effects: &OPTION_BUY_2,
            gate: EventGate::None,
        },
        EventOption {
            label: "[Buy 3 Potions] Lose 40 Gold.",
            effects: &OPTION_BUY_3,
            gate: EventGate::None,
        },
        EventOption {
            label: leave_label,
            effects: leave_effects,
            gate: EventGate::None,
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 4] = options(OPTION_LEAVE_BASE, "[Leave] Nothing happens.");
static OPTIONS_ALL_A15: [EventOption; 4] =
    options(OPTION_LEAVE_A15, "[Leave] Lose 5% of your Max HP.");

// Export event
static EVENT_THE_WOMAN_IN_BLUE_BASE: Entity =
    make_entity_event(EventName::TheWomanInBlue, &OPTIONS_ALL_BASE);
static EVENT_THE_WOMAN_IN_BLUE_A15: Entity =
    make_entity_event(EventName::TheWomanInBlue, &OPTIONS_ALL_A15);
pub fn spawn_event_the_woman_in_blue(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_THE_WOMAN_IN_BLUE_BASE
    } else {
        EVENT_THE_WOMAN_IN_BLUE_A15
    }
}
