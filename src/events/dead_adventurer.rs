use rand::Rng;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::DeadAdventurerReward;
use crate::events::EVENT_CONSUME_EFFECT;
use crate::events::EventPayload;
use crate::game::GameState;
use crate::types::MonsterEncounter;

// Search: escalating elite-return chance; the AdventurerSearch processor grants
// the pre-rolled loot, advances the search count, and consumes after the third find
const OPTION_SEARCH: &[Effect] = &[Effect {
    kind: EffectKind::AdventurerSearch,
    id_source: None,
    target: Target::Direct(None),
}];

// Escape
const OPTION_ESCAPE: &[Effect] = &[EVENT_CONSUME_EFFECT];

pub const LABELS: &[&str] = &[
    "[Search] Find loot; the telegraphed elite may return.",
    "[Escape] Leave with what you found.",
];

pub fn spawn_event_dead_adventurer(state: &mut GameState) -> EventPayload {
    // Which elite returns (telegraphed in the snapshot); rewards are a shrinking
    // pool: each miss draws one uniformly and removes it, which matches the
    // source's shuffled-order-consumed-in-sequence distribution
    let encounter = match state.rng.random_range(0..3) {
        0 => MonsterEncounter::ThreeSentries,
        1 => MonsterEncounter::GremlinNob,
        2 => MonsterEncounter::Lagavulin,
        roll => unreachable!("adventurer enemy roll out of range: {roll}"),
    };
    EventPayload::DeadAdventurer {
        encounter,
        rewards: [
            DeadAdventurerReward::Gold,
            DeadAdventurerReward::Nothing,
            DeadAdventurerReward::Relic,
        ],
        rewards_len: 3,
        searches: 0,
    }
}

pub fn push_option_effects(buf: &mut Vec<Effect>, idx: usize) {
    buf.extend_from_slice(match idx {
        0 => OPTION_SEARCH,
        1 => OPTION_ESCAPE,
        _ => unreachable!("dead adventurer option out of range: {idx}"),
    });
}
