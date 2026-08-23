use rand::Rng;

use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventOptionTemplate;
use crate::events::bake_options;
use crate::events::make_event_option_template;
use crate::game::GameState;
use crate::relics::iter_owned_relics;
use crate::types::RelicName;

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
    EVENT_CONSUME_EFFECT,
];

pub static OPTIONS: &[EventOptionTemplate] = &[
    make_event_option_template(
        "[Offer a Relic] Lose the chosen Relic. Obtain N'loth's Gift.",
        OPTION_TRADE,
    ),
    make_event_option_template("[Refuse] Nothing happens.", &[EVENT_CONSUME_EFFECT]),
];

// Two distinct owned Relics, staked on the context for the trade to pick from
pub fn spawn_event_nloth(state: &mut GameState) -> Vec<usize> {
    let owned: Vec<(RelicName, usize)> = iter_owned_relics(&state.id_relics).collect();
    let idx = state.rng.random_range(0..owned.len());
    let mut jdx = state.rng.random_range(0..owned.len() - 1);
    if jdx >= idx {
        jdx += 1;
    }
    state.event.id_roll_relic.push(owned[idx].1);
    state.event.id_roll_relic.push(owned[jdx].1);
    bake_options(state, OPTIONS)
}
