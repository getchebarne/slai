use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::process_effect_shop_build::restock_card;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

pub fn process_effect_shop_buy_card(id_target: Option<usize>, state: &mut GameState) {
    // Find and remove the shop entry
    let id_card = id_target.expect("ShopBuyCard requires id_target");
    let Mode::Shop { shop_id_cards, .. } = &mut state.mode else {
        unreachable!("ShopBuyCard outside Shop mode")
    };
    let idx = shop_id_cards
        .iter()
        .position(|&id| id == id_card)
        .expect("bought card is a shop entry");
    shop_id_cards.remove(idx);
    let price = state.entities[id_card].price;

    // The Courier: the emptied slot restocks with a same-kind card
    if has_relic(&state.id_relics, RelicName::TheCourier) {
        let bought = &state.entities[id_card];
        let (color, kind, rarity) = (bought.card_color, bought.card_kind, bought.card_rarity);
        let mut id_cards = std::mem::take(shop_id_cards);
        restock_card(state, &mut id_cards, idx, color, kind, rarity);
        let Mode::Shop { shop_id_cards, .. } = &mut state.mode else {
            unreachable!("ShopBuyCard outside Shop mode")
        };
        *shop_id_cards = id_cards;
    }

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
