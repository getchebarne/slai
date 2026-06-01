use crate::consts::SHOP_PURGE_COST_INCREMENT;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::GoldDeltaKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_shop_purge(state: &mut GameState, idx: usize) {
    let id_card = state.id_deck[idx];
    let cost = state.shop_purge_cost;

    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            kind: GoldDeltaKind::Fixed(cost),
        },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    flush_effects_from_buf_to_queue_front(state);

    state.shop_purge_cost += SHOP_PURGE_COST_INCREMENT;
}
