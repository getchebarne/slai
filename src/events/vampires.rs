use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::TARGET_CHARACTER;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EOT_LEAVE;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
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

// Accept: 30% max HP for five Bites
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
    EFFECT_EVENT_CONSUME,
];

// Accept with Blood Vial: it is consumed instead
const OPTION_VIAL: [Effect; 4] = [
    Effect {
        kind: EffectKind::RelicLose,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventRollRelic,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Single,
        },
    },
    EFFECT_PURGE_STRIKES,
    EFFECT_GAIN_BITES,
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_ACCEPT),
    make_event_option_template(&OPTION_VIAL),
    EOT_LEAVE,
];

pub fn option_available(state: &GameState, idx: usize) -> bool {
    match idx {
        1 => has_relic(&state.id_relics, RelicName::BloodVial),
        _ => true,
    }
}

// The Vial option consumes the staked Relic; availability gates it on ownership
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    if let Some(id) = state.id_relics[RelicName::BloodVial as usize] {
        state.event.id_roll_relic.push(id);
    }
    bake_options(state, catalog(state.ascension))
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}
