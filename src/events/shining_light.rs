use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::CandidatePoolDeckFilter;
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

// Enter
const fn enter(numerator: u8, denominator: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Loss,
                // Rounded, not truncated: the source rounds this one damage roll
                amount: Amount::RelativeRounded {
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
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck {
                    filter: CandidatePoolDeckFilter::Upgradeable,
                },
                selection_kind: SelectionKind::Random { count: 2 },
            },
        },
        EVENT_CONSUME_EFFECT,
    ]
}
static OPTION_ENTER_BASE: [Effect; 3] = enter(1, 5);
static OPTION_ENTER_A15: [Effect; 3] = enter(3, 10); // 20% -> 30% max HP loss

// Leave
const OPTION_LEAVE: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options
const fn options(enter_effects: &'static [Effect], enter_label: &'static str) -> [EventOption; 2] {
    [
        EventOption {
            label: enter_label,
            effects: enter_effects,
            gate: EventGate::HasUpgradableInDeck,
        },
        EventOption {
            label: "[Leave] Nothing happens.",
            effects: OPTION_LEAVE,
            gate: EventGate::None,
        },
    ]
}
static OPTIONS_ALL_BASE: [EventOption; 2] = options(
    &OPTION_ENTER_BASE,
    "[Enter] Upgrade 2 random cards. Lose 20% of your max HP.",
);
static OPTIONS_ALL_A15: [EventOption; 2] = options(
    &OPTION_ENTER_A15,
    "[Enter] Upgrade 2 random cards. Lose 30% of your max HP.",
);

// Export event
static EVENT_SHINING_LIGHT_BASE: Entity =
    make_entity_event(EventName::ShiningLight, &OPTIONS_ALL_BASE);
static EVENT_SHINING_LIGHT_A15: Entity =
    make_entity_event(EventName::ShiningLight, &OPTIONS_ALL_A15);
pub fn spawn_event_shining_light(ascension: u8) -> Entity {
    if ascension < 15 {
        EVENT_SHINING_LIGHT_BASE
    } else {
        EVENT_SHINING_LIGHT_A15
    }
}
