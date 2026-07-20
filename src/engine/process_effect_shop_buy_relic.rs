use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_shop_buy_relic(id_target: Option<usize>, state: &mut GameState) {
    // Find and remove the shop entry
    let id_relic = id_target.expect("ShopBuyRelic requires id_target");
    let Mode::Shop(shop) = &mut state.mode else {
        unreachable!("ShopBuyRelic outside Shop mode")
    };
    let idx = shop
        .id_relics
        .iter()
        .position(|&id| id == id_relic)
        .expect("bought relic is a shop entry");
    shop.id_relics.remove(idx);
    let price = shop.relic_prices.remove(idx);

    // Charge gold and grant the relic
    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(price),
        },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::RelicAdopt,
        id_source: None,
        target: Target::Direct(Some(id_relic)),
    });
    flush_effects_from_buf_to_queue_front(state);
}
