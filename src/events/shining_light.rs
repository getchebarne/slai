use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::CandidatePool;
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

// A15+: HP loss 20% max → 30% max

const fn enter(numer: u8, denom: u8) -> [Effect; 3] {
    [
        Effect {
            kind: EffectKind::HealthDelta {
                sign: HealthDeltaSign::Loss,
                amount: HealthDeltaAmount::Pct { numer, denom },
            },
            id_source: None,
            target: Target::Direct(None),
        },
        Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck { filter: CandidatePoolDeckFilter::Upgradeable },
                selection_kind: SelectionKind::Random { count: 2 },
            },
        },
        EVENT_END_EFFECT,
    ]
}

static ENTER_BASE: [Effect; 3] = enter(1, 5);
static ENTER_A15: [Effect; 3] = enter(3, 10);

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const fn options(enter_effects: &'static [Effect], enter_label: &'static str) -> [EventOption; 2] {
    [
        EventOption {
            label: enter_label,
            effects: enter_effects,
            gate: EventGate::HasUpgradableInDeck,
        },
        EventOption {
            label: "Leave",
            effects: LEAVE,
            gate: EventGate::None,
        },
    ]
}

static OPTIONS_BASE: [EventOption; 2] = options(
    &ENTER_BASE,
    "Enter (lose 20% max HP, upgrade up to 2 random cards)",
);
static OPTIONS_A15: [EventOption; 2] = options(
    &ENTER_A15,
    "Enter (lose 30% max HP, upgrade up to 2 random cards)",
);

pub static SHINING_LIGHT_BASE: Entity = make_entity_event(EventName::ShiningLight, &OPTIONS_BASE);
pub static SHINING_LIGHT_A15: Entity = make_entity_event(EventName::ShiningLight, &OPTIONS_A15);

pub fn spawn_event_shining_light(ascension: u8) -> Entity {
    if ascension < 15 {
        SHINING_LIGHT_BASE
    } else {
        SHINING_LIGHT_A15
    }
}
