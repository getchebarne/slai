use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::GoldDeltaKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_shop_buy_relic(id_target: Option<usize>, state: &mut GameState) {
    let id_relic = id_target.expect("ShopBuyRelic requires id_target");
    let idx = state
        .shop_id_relics
        .iter()
        .position(|&id| id == id_relic)
        .expect("bought relic is a shop entry");
    state.shop_id_relics.remove(idx);
    let price = state.shop_relic_prices.remove(idx);
    let name = state.entities[id_relic].relic_name;

    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            kind: GoldDeltaKind::Fixed(price),
        },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::RelicGrantSpecific {
            name,
            fallback_circlet: false,
        },
        id_source: None,
        target: Target::Direct(None),
    });
    flush_effects_from_buf_to_queue_front(state);
}
