use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::entity::Entity;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::OPTION_LEAVE;
use crate::events::make_entity_event_option;
use crate::game::GameState;
use crate::types::CardName;
use crate::types::CardPile;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

// Every Strike leaves the deck, five Bites enter it
const EFFECT_PURGE_STRIKES: Effect = Effect {
    kind: EffectKind::CardPurge,
    id_source: None,
    target: Target::Resolve {
        candidate_pool: CandidatePool::Deck,
        filter: CandidateFilter::StarterStrike,
        selection_kind: SelectionKind::All,
    },
};

const EFFECT_GAIN_BITES: Effect = Effect {
    kind: EffectKind::CardAdd {
        card_name: CardName::Bite,
        pile: CardPile::Deck,
        count: 5,
        upgraded: false,
    },
    id_source: None,
    target: Target::Direct(None),
};

const OPTION_ACCEPT: &[Effect] = &[
    Effect {
        kind: EffectKind::MaxHealthDelta {
            sign: DeltaSign::Loss,
            amount: Amount::RelativeCeil {
                numerator: 3,
                denominator: 10,
            },
        },
        id_source: None,
        target: TARGET_CHARACTER,
    },
    EFFECT_PURGE_STRIKES,
    EFFECT_GAIN_BITES,
    EVENT_CONSUME_EFFECT,
];

const OPTION_VIAL: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicLose {
            name: RelicName::BloodVial,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_PURGE_STRIKES,
    EFFECT_GAIN_BITES,
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[Entity] = &[
    make_entity_event_option(
        "[Accept] Lose 30% of your Max HP. Replace your Strikes with 5 Bites.",
        OPTION_ACCEPT,
    ),
    make_entity_event_option(
        "[Offer Blood Vial] Replace your Strikes with 5 Bites.",
        OPTION_VIAL,
    ),
    OPTION_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        1 => has_relic(&state.id_relics, RelicName::BloodVial),
        _ => true,
    }
}
