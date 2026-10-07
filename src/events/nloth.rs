use rand::Rng;

use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EFFECT_EVENT_CONSUME;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::relics::iter_owned_relics;
use crate::types::RelicName;
use crate::utils::has_relic;

// One trade for both offers: the player picks which staked Relic to give up
const OPTION_TRADE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicLose,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::EventRollRelic,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::NlothsGift,
            fallback_circlet: true,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Trading with N'loth's Gift owned: the offered Relic stays and the prize falls back to Circlet
const OPTION_TRADE_PRIZE_OWNED: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::NlothsGift,
            fallback_circlet: true,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

// Catalog layout: Trade, Leave, then the trade with its prize owned
pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_TRADE),
    make_event_option_template(&[EFFECT_EVENT_CONSUME]),
    make_event_option_template(OPTION_TRADE_PRIZE_OWNED),
];
const IDX_TRADE: usize = 0;
const IDX_LEAVE: usize = 1;
const IDX_TRADE_PRIZE_OWNED: usize = 2;

// Two distinct owned Relics, staked on the context for the trade to pick from unless the Gift is owned
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    // Collect owned `RelicName`s and their IDs
    let relic_name_id_owned: Vec<(RelicName, usize)> =
        iter_owned_relics(&state.id_relics).collect();

    // Roll picks
    let idx = state.rng.random_range(0..relic_name_id_owned.len());
    let mut jdx = state.rng.random_range(0..relic_name_id_owned.len() - 1);
    if jdx >= idx {
        jdx += 1;
    }

    // Push and bake Event Options
    state.event.id_roll_relic.push(relic_name_id_owned[idx].1);
    state.event.id_roll_relic.push(relic_name_id_owned[jdx].1);
    let eots = catalog(state.ascension);
    let idx_trade = if has_relic(&state.id_relics, RelicName::NlothsGift) {
        IDX_TRADE_PRIZE_OWNED
    } else {
        IDX_TRADE
    };
    bake_options(state, &[eots[idx_trade], eots[IDX_LEAVE]])
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
