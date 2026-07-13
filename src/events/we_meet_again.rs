use crate::effect::Amount;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::EntityKind;
use crate::entity::make_entity_event;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::DeltaSign;
use crate::types::EventName;

const RELIC_REWARD: Effect = Effect {
    kind: EffectKind::RelicGrantRandom,
    id_source: None,
    target: Target::Direct(None),
};

// The offered potion/card/gold are rolled at event entry (see roll_event_entry_picks)
// and exposed on the snapshot; options resolve them through the EventPick* pools
const OPTION_GIVE_POTION: &[Effect] = &[
    Effect {
        kind: EffectKind::PotionDiscard,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventPickPotion,
            selection_kind: SelectionKind::Single,
        },
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

const OPTION_GIVE_GOLD: &[Effect] = &[
    Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::EventGoldRolled,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

const OPTION_GIVE_CARD: &[Effect] = &[
    Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventPickCard,
            selection_kind: SelectionKind::Single,
        },
    },
    RELIC_REWARD,
    EVENT_CONSUME_EFFECT,
];

// Attack
const OPTION_ATTACK: &[Effect] = &[EVENT_CONSUME_EFFECT];

// All options
const OPTIONS_ALL: &[EventOption] = &[
    EventOption {
        label: "[Give Potion] Lose the offered potion. Obtain a random relic.",
        effects: OPTION_GIVE_POTION,
        gate: EventGate::EventPickValid(EntityKind::Potion),
    },
    EventOption {
        label: "[Give Gold] Lose the asked gold. Obtain a random relic.",
        effects: OPTION_GIVE_GOLD,
        gate: EventGate::GoldAtLeast(50),
    },
    EventOption {
        label: "[Give Card] Lose the offered card. Obtain a random relic.",
        effects: OPTION_GIVE_CARD,
        gate: EventGate::EventPickValid(EntityKind::Card),
    },
    EventOption {
        label: "[Attack] Nothing happens.",
        effects: OPTION_ATTACK,
        gate: EventGate::None,
    },
];

// Export event
static EVENT_WE_MEET_AGAIN: Entity = make_entity_event(EventName::WeMeetAgain, OPTIONS_ALL);
pub fn spawn_event_we_meet_again() -> Entity {
    EVENT_WE_MEET_AGAIN
}
