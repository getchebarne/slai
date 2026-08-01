use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::Entity;
use crate::entity::make_entity_event_option;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::MonsterName;
use crate::types::RelicName;

const SPAWN_FUNGI: Effect = Effect {
    kind: EffectKind::MonsterSpawn {
        name: MonsterName::FungiBeast,
    },
    id_source: None,
    target: Target::Direct(None),
};

// Stomp: fight 3 Fungi Beasts — the reward roll gives gold and an Odd Mushroom
const OPTION_STOMP: &[Effect] = &[
    EVENT_CONSUME_EFFECT,
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    SPAWN_FUNGI,
    Effect {
        kind: EffectKind::CombatStart {
            event_gold: Some(Amount::Range { min: 20, max: 30 }),
            event_relic: Some(RelicName::OddMushroom),
            event_relic_roll: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
];

// Eat: heal 25% max HP and become Cursed w/ Parasite
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
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    },
    Effect {
        kind: EffectKind::CardAdd {
            card_name: CardName::Parasite,
            pile: CardPile::Deck,
            count: 1,
            upgraded: false,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option("[Stomp] Fight 3 Fungi Beasts.", OPTION_STOMP),
    make_entity_event_option(
        "[Eat] Heal 25% of your Max HP. Become Cursed - Parasite.",
        OPTION_EAT,
    ),
];
