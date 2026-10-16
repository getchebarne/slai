use rand::Rng;

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
use crate::types::RelicName;

// One trade for both offers: the player selects which staked Relic to give up
const OPTION_TRADE: &[Effect] = &[
    Effect {
        kind: EffectKind::RelicLose,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::RelicEventRoll,
            filters: &[],
            selection_kind: SelectionKind::Input { count: 1 },
        },
    },
    Effect {
        kind: EffectKind::RelicGrantSpecific {
            name: RelicName::NlothsGift,
        },
        id_source: None,
        target: Target::Direct(None),
    },
    EFFECT_EVENT_CONSUME,
];

pub static EOTS_BASE: &[EventOptionTemplate] = &[
    make_event_option_template(OPTION_TRADE),
    make_event_option_template(&[EFFECT_EVENT_CONSUME]),
];

// Two distinct owned Relics, staked on the context for the trade to select from
pub fn spawn(state: &mut GameState) -> Vec<usize> {
    // Collect owned Relics
    let id_relics: Vec<usize> = state.id_relics.iter().flatten().copied().collect();

    // Roll the two staked Relics
    let idx = state.rng.random_range(0..id_relics.len());
    let mut jdx = state.rng.random_range(0..id_relics.len() - 1);
    if jdx >= idx {
        jdx += 1;
    }

    // Push and bake Event Options
    state.event.id_relic_event_roll.push(id_relics[idx]);
    state.event.id_relic_event_roll.push(id_relics[jdx]);
    bake_options(state, catalog(state.ascension))
}

pub fn catalog(_ascension: u8) -> &'static [EventOptionTemplate] {
    EOTS_BASE
}

pub fn option_available(_state: &GameState, _idx: usize) -> bool {
    true
}
