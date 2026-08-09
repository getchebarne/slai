use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::shop::apply_shop_discounts;
use crate::engine::shop::make_card_colored;
use crate::engine::shop::make_card_colorless;
use crate::engine::shop::make_potion;
use crate::engine::shop::restock_relic;
use crate::game::GameState;
use crate::types::CardColor;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::types::ShopSlot;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::mode_top_mut;

pub fn process_effect_shop_buy(id_target: Option<usize>, state: &mut GameState, slot: ShopSlot) {
    let id_bought = id_target.expect("ShopBuy requires id_target");
    let Mode::Shop {
        shop_id_cards,
        shop_id_relics,
        shop_id_potions,
        shop_purge_cost,
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("ShopBuy outside Shop mode")
    };

    // Take the entry out of its slot
    let idx = {
        let ids: &mut Vec<usize> = match slot {
            ShopSlot::Card => shop_id_cards,
            ShopSlot::Relic => shop_id_relics,
            ShopSlot::Potion => shop_id_potions,
        };
        let idx = ids
            .iter()
            .position(|&id| id == id_bought)
            .expect("Bought entry is a shop entry");
        ids.remove(idx);
        idx
    };

    // Snapshot bought price; the Relic name is only meaningful on the Relic slot
    let price_bought = state.entities[id_bought].price;
    let name_bought = (slot == ShopSlot::Relic).then(|| state.entities[id_bought].relic_name);

    // Membership Card bought mid-shop: retro-discount the remaining stock and purge
    if name_bought == Some(RelicName::MembershipCard) {
        for &id in shop_id_cards
            .iter()
            .chain(shop_id_relics.iter())
            .chain(shop_id_potions.iter())
        {
            let price = &mut state.entities[id].price;
            *price = ((*price as u32 + 1) / 2) as u16;
        }

        // Smiling Mask: the fixed purge cost is exempt from discounts
        if !has_relic(&state.id_relics, RelicName::SmilingMask) {
            *shop_purge_cost = (*shop_purge_cost + 1) / 2;
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
                    make_card_colorless(&mut state.entities, &mut state.rng, shop_id_cards, rarity)
                } else {
                    make_card_colored(&mut state.entities, &mut state.rng, shop_id_cards, kind)
                };
                state.entities[id_new].price =
                    apply_shop_discounts(state.entities[id_new].price, &state.id_relics);
                shop_id_cards.insert(idx, id_new);
            }
            ShopSlot::Potion => {
                let id_new = make_potion(&mut state.entities, &mut state.rng);
                state.entities[id_new].price =
                    apply_shop_discounts(state.entities[id_new].price, &state.id_relics);
                shop_id_potions.insert(idx, id_new);
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
                    shop_id_relics,
                    idx,
                );
            }
        }
    }

    // Charge gold and hand the entity to its Adopt effect
    let kind_adopt = match slot {
        ShopSlot::Card => EffectKind::CardAddToDeck,
        ShopSlot::Relic => EffectKind::RelicAdopt,
        ShopSlot::Potion => EffectKind::PotionAdopt,
    };
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
        kind: kind_adopt,
        id_source: None,
        target: Target::Direct(Some(id_bought)),
    });
    flush_effects_from_buf_to_queue_front(state);
}
