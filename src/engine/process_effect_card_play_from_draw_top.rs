use crate::effect::CandidatePool;
use crate::effect::CandidatePoolMonstersFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::entity::CostOverride;
use crate::game::GameState;
use crate::types::CostScope;
use crate::types::Mode;

// StS PlayTopCardAction: lift the top card (no draw, no on-draw hooks) and autoplay it
// for free at a random monster; empty draw pile re-queues itself behind a reshuffle
pub fn process_effect_card_play_from_draw_top(state: &mut GameState) {
    let Mode::Combat {
        id_pile_draw,
        id_pile_discard,
        ..
    } = &mut state.mode
    else {
        unreachable!("process_effect_card_play_from_draw_top outside Combat mode")
    };
    if id_pile_draw.is_empty() {
        if id_pile_discard.is_empty() {
            return;
        }
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
    state.entities[id_card].card_cost_override = Some(CostOverride {
        amount: 0,
        scope: CostScope::UntilPlayed,
    });

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
            candidate_pool: CandidatePool::Monsters {
                filter: CandidatePoolMonstersFilter::All,
            },
            selection_kind: SelectionKind::Random { count: 1 },
        },
    });
}
