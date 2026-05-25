use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeckSelectKind;

// Push a DeckSelectPick halt at the queue front. The resolver computes
// candidates via CandidatePool::DeckFiltered(kind); if the filter yields
// an empty set, resolve_selection_kind short-circuits and the pick is skipped
pub fn process_effect_deck_select_start(state: &mut GameState, kind: DeckSelectKind) {
    state.effect_queue.push_front(Effect {
        kind: EffectKind::DeckSelectPick { kind },
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::DeckFiltered(kind),
            selection_kind: SelectionKind::Input { count: 1 },
        },
    });
}
