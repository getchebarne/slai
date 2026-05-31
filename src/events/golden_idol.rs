use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::HealthDeltaAmount;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::RelicName;

// Take
const OPTION_TAKE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::GoldenIdol,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    Effect {
        kind: EffectKind::EventAdvanceState { delta: 1 }, // Outrun / Smash / Hide
        id_source: None,
        target: Target::Direct(None),
    },
];

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// Outrun
const OPTION_OUTRUN: &[Effect] = &[
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Injury,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// Smash
const fn smash(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                amount: HealthDeltaAmount::Relative {
                    numerator,
                    denominator,
                },
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_SMASH_BASE: [Effect; 2] = smash(1, 4);
static OPTION_SMASH_A15: [Effect; 2] = smash(35, 100); // 25% -> 35% max HP loss

// Hide
const fn hide(numerator: u8, denominator: u8) -> [Effect; 2] {
    [
        Effect {
            kind: EffectKind::MaxHealthDelta {
                sign: DeltaSign::Loss,
                amount: HealthDeltaAmount::Relative {
                    numerator,
                    denominator,
                },
            },
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Character,
                selection_kind: SelectionKind::Single,
            },
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_HIDE_BASE: [Effect; 2] = hide(8, 100);
static OPTION_HIDE_A15: [Effect; 2] = hide(10, 100); // 8% -> 10% max HP cap loss

// All options
const fn options(
    smash_effects: &'static [Effect],
    smash_label: &'static str,
    hide_effects: &'static [Effect],
    hide_label: &'static str,
) -> [EventOption; 5] {
    [
        EventOption {
            label: "[Take] Obtain Golden Idol.",
            effects: OPTION_TAKE,
            gate: EventGate::EventStateEq(0),
        },
        EventOption {
            label: "[Leave] Nothing happens.",
            effects: OPTION_LEAVE,
            gate: EventGate::EventStateEq(0),
        },
        EventOption {
            label: "[Outrun] Become Cursed - Injury.",
            effects: OPTION_OUTRUN,
            gate: EventGate::EventStateEq(1),
        },
        EventOption {
            label: smash_label,
            effects: smash_effects,
            gate: EventGate::EventStateEq(1),
        },
        EventOption {
            label: hide_label,
            effects: hide_effects,
            gate: EventGate::EventStateEq(1),
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 5] = options(
    &OPTION_SMASH_BASE,
    "[Smash] Take 25% of your max HP as damage.",
    &OPTION_HIDE_BASE,
    "[Hide] Lose 8% of your max HP.",
);
static OPTIONS_ALL_A15: [EventOption; 5] = options(
    &OPTION_SMASH_A15,
    "[Smash] Take 35% of your max HP as damage.",
    &OPTION_HIDE_A15,
    "[Hide] Lose 10% of your max HP.",
);

// Export event
static EVENT_GOLDEN_IDOL_BASE: Entity = make_entity_event(EventName::GoldenIdol, &OPTIONS_ALL_BASE);
static EVENT_GOLDEN_IDOL_A15: Entity = make_entity_event(EventName::GoldenIdol, &OPTIONS_ALL_A15);
pub fn spawn_event_golden_idol(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_GOLDEN_IDOL_BASE
    } else {
        EVENT_GOLDEN_IDOL_A15
    }
}
