use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_shop_buy_potion(id_target: Option<usize>, state: &mut GameState) {
    // Find and remove the shop entry
    let id_potion = id_target.expect("ShopBuyPotion requires id_target");
    let Mode::Shop(shop) = &mut state.mode else {
        unreachable!("ShopBuyPotion outside Shop mode")
    };
    let idx = shop
        .id_potions
        .iter()
        .position(|&id| id == id_potion)
        .expect("bought potion is a shop entry");
    shop.id_potions.remove(idx);
    let price = shop.potion_prices.remove(idx);

    // Charge gold and slot the potion into the belt
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
        kind: EffectKind::PotionAdopt,
        id_source: None,
        target: Target::Direct(Some(id_potion)),
    });
    flush_effects_from_buf_to_queue_front(state);
}
