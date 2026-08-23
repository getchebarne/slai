use crate::consts::SHOP_PURGE_COST_INCREMENT;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Focus;
use crate::utils::context_focus;
use crate::utils::flush_effects_from_buf_to_queue_front;

// Pay-then-pick: charge the purge cost, then halt on the deck pick
pub fn process_effect_shop_purge(state: &mut GameState) {
    assert!(
        context_focus(state) == Focus::Shop,
        "ShopPurge outside the Shop context"
    );
    let cost = state.shop.purge_cost;

    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(cost),
        },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::Deck,
            filter: CandidateFilter::Purgeable,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    });
    flush_effects_from_buf_to_queue_front(state);

    // Ramps for the rest of the run; the next shop build reads the new value
    state.shop_purge_cost_run += SHOP_PURGE_COST_INCREMENT;

    // A shop's Card removal can be used once per visit
    state.shop.purged = true;
}
