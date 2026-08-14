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
use crate::types::Frame;
use crate::types::RelicName;
use crate::types::ShopSlot;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::frame_top_mut;
use crate::utils::has_relic;

pub fn process_effect_shop_buy(id_target: Option<usize>, state: &mut GameState, slot: ShopSlot) {
    let id_bought = id_target.expect("ShopBuy requires id_target");
    let Frame::Shop {
        cards,
        relics,
        potions,
        purge_cost,
        ..
    } = frame_top_mut(&mut state.frame_stack)
    else {
        unreachable!("ShopBuy outside the Shop frame")
    };

    // Take the offer out of its slot; its price settles the sale
    let (idx, price_bought) = {
        let offers: &mut Vec<(usize, u16)> = match slot {
            ShopSlot::Card => cards,
            ShopSlot::Relic => relics,
            ShopSlot::Potion => potions,
        };
        let idx = offers
            .iter()
            .position(|&(id, _)| id == id_bought)
            .expect("Bought entry is a shop entry");
        let (_, price) = offers.remove(idx);
        (idx, price)
    };

    // The Relic name is only meaningful on the Relic slot
    let name_bought = (slot == ShopSlot::Relic).then(|| state.entities[id_bought].relic_name);

    // Membership Card bought mid-shop: retro-discount the remaining stock and purge
    if name_bought == Some(RelicName::MembershipCard) {
        for (_, price) in cards
            .iter_mut()
            .chain(relics.iter_mut())
            .chain(potions.iter_mut())
        {
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
                let (id_new, price) = if color == CardColor::Colorless {
                    make_card_colorless(&mut state.entities, &mut state.rng, cards, rarity)
                } else {
                    make_card_colored(&mut state.entities, &mut state.rng, cards, kind)
                };
                let price = apply_shop_discounts(price, &state.id_relics);
                cards.insert(idx, (id_new, price));
            }
            ShopSlot::Potion => {
                let (id_new, price) = make_potion(&mut state.entities, &mut state.rng);
                let price = apply_shop_discounts(price, &state.id_relics);
                potions.insert(idx, (id_new, price));
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
                    relics,
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
