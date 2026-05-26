use crate::effect::CandidatePoolDeckFilter;
use crate::effect::Effect;
use crate::effect::Target;
use crate::effect::EffectKind;
use crate::effect::CandidatePool;
use crate::effect::SelectionKind;
use crate::entity::Entity;
use crate::entity::make_entity_event;
use crate::events::EVENT_END_EFFECT;
use crate::events::EventGate;
use crate::events::EventOption;
use crate::types::EventName;

const PRAY: &[Effect] = &[
    Effect {
            kind: EffectKind::CardUpgrade,
            id_source: None,
            target: Target::Resolve {
                candidate_pool: CandidatePool::Deck { filter: CandidatePoolDeckFilter::Upgradeable },
                selection_kind: SelectionKind::Input { count: 1 },
            },
        },
    EVENT_END_EFFECT,
];

const LEAVE: &[Effect] = &[EVENT_END_EFFECT];

const OPTIONS: &[EventOption] = &[
    EventOption {
        label: "Pray (upgrade a card)",
        effects: PRAY,
        gate: EventGate::HasUpgradableInDeck,
    },
    EventOption {
        label: "Leave",
        effects: LEAVE,
        gate: EventGate::None,
    },
];

pub static UPGRADE_SHRINE: Entity = make_entity_event(EventName::UpgradeShrine, OPTIONS);
