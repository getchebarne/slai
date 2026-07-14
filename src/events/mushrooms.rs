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
use crate::types::CardName;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::MonsterName;

const SPAWN_FUNGI: Effect = Effect {
    kind: EffectKind::MonsterSpawn {
        name: MonsterName::FungiBeast,
    },
    id_source: None,
    target: Target::Direct(None),
};

// Stomp: fight 3 Fungi Beasts; the reward roll injects 20-30 gold + Odd Mushroom
const OPTION_STOMP: &[Effect] = &[
    EVENT_CONSUME_EFFECT,
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    Effect {
        kind: EffectKind::CombatStart,
        id_source: None,
        target: Target::Direct(None),
    },
];

// Eat: heal 25% max HP (truncated), become Cursed - Parasite
const OPTION_EAT: &[Effect] = &[
    Effect {
        kind: EffectKind::HealthDelta {
            sign: DeltaSign::Gain,
            amount: Amount::Relative {
                numerator: 1,
                denominator: 4,
            },
        },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Character,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::CardAddToDeck {
            card_name: CardName::Parasite,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Stomp] Fight 3 Fungi Beasts.",
        effects: OPTION_STOMP,
        gate: EventGate::None,
    },
    EventOption {
        label: "[Eat] Heal 25% of your Max HP. Become Cursed - Parasite.",
        effects: OPTION_EAT,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_MUSHROOMS: Entity = make_entity_event(EventName::Mushrooms, OPTIONS_ALL);
pub fn spawn_event_mushrooms() -> Entity {
    EVENT_MUSHROOMS
}
