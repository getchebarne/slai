use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_shop_buy_card(id_target: Option<usize>, state: &mut GameState) {
    // Find and remove the shop entry
    let id_card = id_target.expect("ShopBuyCard requires id_target");
    let Mode::Shop(shop) = &mut state.mode else {
        unreachable!("ShopBuyCard outside Shop mode")
    };
    let idx = shop
        .id_cards
        .iter()
        .position(|&id| id == id_card)
        .expect("bought card is a shop entry");
    shop.id_cards.remove(idx);
    let price = shop.card_prices.remove(idx);

    // Charge gold and add the card to the deck
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
        kind: EffectKind::CardAdopt,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    flush_effects_from_buf_to_queue_front(state);
}
