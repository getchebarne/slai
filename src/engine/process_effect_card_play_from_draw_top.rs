use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::CostScope;
use crate::types::Mode;

pub fn process_effect_card_play_from_draw_top(state: &mut GameState) {
    let Some(Mode::Combat {
        id_pile_draw,
        id_pile_discard,
        ..
    }) = state.mode_stack.last_mut()
    else {
        unreachable!("process_effect_card_play_from_draw_top outside Combat mode")
    };

    // Check if the draw pile is empty
    if id_pile_draw.is_empty() {
        if id_pile_discard.is_empty() {
            return;
        }

        // Executes in reverse:
        //     1. ShuffleDiscardPileIntoDrawPile
        //     2. CardPlayFromDrawTop (re-queued)
        state.effect_queue.push_front(Effect {
            kind: EffectKind::CardPlayFromDrawTop,
            id_source: None,
            target: Target::Direct(None),
        });
        state.effect_queue.push_front(Effect {
            kind: EffectKind::ShuffleDiscardPileIntoDrawPile,
            id_source: None,
            target: Target::Direct(None),
        });
        return;
    }

    // Detached from the pile here; card_play's routing effects move it onward
    let id_card = id_pile_draw.pop().unwrap();

    // Executes in reverse:
    //     1. SetCostOverride
    //     2. TargetSet
    //     3. CardPlay
    //     4. TargetClear
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TargetClear,
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardPlay,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TargetSet,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Monsters,
            filter: CandidateFilter::Any,
            selection_kind: SelectionKind::Random { count: 1 }, // Select target randomly
        },
    });
    state.effect_queue.push_front(Effect {
        kind: EffectKind::SetCostOverride {
            amount: 0,
            only_reduce: false,
            random: false,
            scope: CostScope::UntilPlayed,
        },
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
