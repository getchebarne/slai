use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::shop::restock_card;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::mode_top_mut;

pub fn process_effect_shop_buy_card(id_target: Option<usize>, state: &mut GameState) {
    // Find and remove the shop entry
    let id_card = id_target.expect("ShopBuyCard requires id_target");
    let Mode::Shop { shop_id_cards, .. } = mode_top_mut(&mut state.mode_stack) else {
        unreachable!("ShopBuyCard outside Shop mode")
    };

    // Take Card
    let idx = shop_id_cards
        .iter()
        .position(|&id| id == id_card)
        .expect("Bought Card is a shop entry");
    shop_id_cards.remove(idx);

    // Snapshot bought price
    let price_bought = state.entities[id_card].price;

    // The Courier: the emptied slot restocks with a same-kind Card
    if has_relic(&state.id_relics, RelicName::TheCourier) {
        let card_bought = &state.entities[id_card];
        let (color, kind, rarity) = (
            card_bought.card_color,
            card_bought.card_kind,
            card_bought.card_rarity,
        );
        restock_card(
            &mut state.entities,
            &mut state.rng,
            &state.id_relics,
            shop_id_cards,
            idx,
            color,
            kind,
            rarity,
        );
    }

    // Charge gold and add the Card to the deck
    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(price_bought),
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
