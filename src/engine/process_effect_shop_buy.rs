use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::effect_gold_loss;
use crate::engine::shop::apply_shop_discounts;
use crate::engine::shop::make_card_colored;
use crate::engine::shop::make_card_colorless;
use crate::engine::shop::make_potion;
use crate::engine::shop::restock_relic;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::Focus;
use crate::types::RelicName;
use crate::types::Shop;
use crate::types::ShopSlot;
use crate::utils::context_focus;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

pub fn process_effect_shop_buy(id_target: Option<usize>, state: &mut GameState, slot: ShopSlot) {
    let id_bought = id_target.expect("ShopBuy requires id_target");
    assert!(
        context_focus(state) == Focus::Shop,
        "ShopBuy outside the Shop context"
    );
    let Shop {
        id_cards,
        id_relics,
        id_potions,
        purge_cost,
        ..
    } = &mut state.shop;

    // Take the offer out of its slot; its price settles the sale and leaves it
    let idx = {
        let offers: &mut Vec<usize> = match slot {
            ShopSlot::Card => id_cards,
            ShopSlot::Relic => id_relics,
            ShopSlot::Potion => id_potions,
        };
        let idx = offers
            .iter()
            .position(|&id| id == id_bought)
            .expect("Bought entry is a shop entry");
        offers.remove(idx);
        idx
    };
    let price_bought = state.entities[id_bought].shop_price;
    state.entities[id_bought].shop_price = 0;

    // The Relic name is only meaningful on the Relic slot
    let name_bought = (slot == ShopSlot::Relic).then(|| state.entities[id_bought].relic_name);

    // Membership Card bought mid-shop: retro-discount the remaining stock and purge
    if name_bought == Some(RelicName::MembershipCard) {
        for &id in id_cards
            .iter()
            .chain(id_relics.iter())
            .chain(id_potions.iter())
        {
            let price = &mut state.entities[id].shop_price;
            *price = ((*price as u32 + 1) / 2) as u16;
        }

        // Smiling Mask: the fixed purge cost is exempt from discounts
        if !has_relic(&state.id_relics, RelicName::SmilingMask) {
            *purge_cost = (*purge_cost + 1) / 2;
        }
    }

    // The Courier: the emptied slot restocks; buying the Courier itself restocks its own slot
    let courier_bought = name_bought == Some(RelicName::TheCourier);
    if courier_bought || has_relic(&state.id_relics, RelicName::TheCourier) {
        match slot {
            ShopSlot::Card => {
                let bought = &state.entities[id_bought];
                let (color, kind, rarity) =
                    (bought.card_color, bought.card_kind, bought.card_rarity);
                let id_new = if color == CardColor::Colorless {
                    make_card_colorless(&mut state.entities, &mut state.rng, id_cards, rarity)
                } else {
                    make_card_colored(
                        &mut state.entities,
                        &mut state.rng,
                        id_cards,
                        kind,
                        state.id_character,
                        &state.id_relics,
                    )
                };
                let price = &mut state.entities[id_new].shop_price;
                *price = apply_shop_discounts(*price, &state.id_relics);
                id_cards.insert(idx, id_new);
            }
            ShopSlot::Potion => {
                let id_new = make_potion(&mut state.entities, &mut state.rng);
                let price = &mut state.entities[id_new].shop_price;
                *price = apply_shop_discounts(*price, &state.id_relics);
                id_potions.insert(idx, id_new);
            }
            ShopSlot::Relic => {
                // Restock as if the sale settled: priced with the bought Relic, never re-offering it
                let mut id_relics_settled = state.id_relics;
                id_relics_settled[name_bought.expect("Relic slot carries a name") as usize] =
                    Some(id_bought);
                restock_relic(
                    &mut state.entities,
                    &mut state.rng,
                    &id_relics_settled,
                    id_relics,
                    idx,
                );
            }
        }
    }

    // Charge gold and hand the entity to its Adopt effect
    let kind_adopt = match slot {
        ShopSlot::Card => EffectKind::CardAdopt,
        ShopSlot::Relic => EffectKind::RelicAdopt,
        ShopSlot::Potion => EffectKind::PotionAdopt,
    };
    state.effect_buf.clear();
    state.effect_buf.push(effect_gold_loss(price_bought));
    state.effect_buf.push(Effect {
        kind: kind_adopt,
        id_source: None,
        target: Target::Direct(Some(id_bought)),
    });
    flush_effects_from_buf_to_queue_front(state);
}
