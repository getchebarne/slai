use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::shop::apply_shop_discounts;
use crate::engine::shop::make_card_restock;
use crate::engine::shop::make_potion;
use crate::engine::shop::restock_relic;
use crate::game::GameState;
use crate::types::DeltaSign;
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

    // Take the offer out of its slot; its price settles the sale
    let id_offers: &mut Vec<usize> = match slot {
        ShopSlot::Card => id_cards,
        ShopSlot::Relic => id_relics,
        ShopSlot::Potion => id_potions,
    };
    let idx = id_offers
        .iter()
        .position(|&id| id == id_bought)
        .expect("Bought entry is a shop entry");
    id_offers.remove(idx);

    // What was paid for it
    let price_bought = state.entities[id_bought]
        .price
        .take()
        .expect("Shop stock carries a price");

    // The Relic name is only meaningful on the Relic slot
    let name_bought = (slot == ShopSlot::Relic).then(|| state.entities[id_bought].relic_name);

    // Membership Card bought mid-shop: retro-discount the remaining stock and purge
    if name_bought == Some(RelicName::MembershipCard) {
        for &id in id_cards.iter().chain(&*id_relics).chain(&*id_potions) {
            let entity = &mut state.entities[id];
            entity.price = entity.price.map(|price| ((price as u32 + 1) / 2) as u16);
        }

        // Smiling Mask: the fixed purge cost is exempt from discounts
        if !has_relic(&state.id_relics, RelicName::SmilingMask) {
            *purge_cost = (state.shop_purge_cost_run + 1) / 2;
        }
    }

    // The Courier: the emptied slot restocks
    if has_relic(&state.id_relics, RelicName::TheCourier) {
        match slot {
            ShopSlot::Card => {
                let bought = &state.entities[id_bought];
                let (color, kind) = (bought.card_color, bought.card_kind);
                let id_card = make_card_restock(state, color, kind);
                state.shop.id_cards.insert(idx, id_card);
            }
            ShopSlot::Potion => {
                let sacred_bark = has_relic(&state.id_relics, RelicName::SacredBark);
                let id_potion = make_potion(&mut state.entities, &mut state.rng, sacred_bark);
                let potion = &mut state.entities[id_potion];
                potion.price = potion
                    .price
                    .map(|price| apply_shop_discounts(price, &state.id_relics));
                id_potions.insert(idx, id_potion);
            }
            ShopSlot::Relic => {
                // Restock as if the sale settled: the bought Relic counts for spawn gates and price
                let mut id_relics_settled = state.id_relics;
                id_relics_settled[name_bought.expect("Relic slot carries a name") as usize] =
                    Some(id_bought);
                restock_relic(state, &id_relics_settled, idx);
            }
        }
    }

    // Charge gold and hand the entity to its Adopt effect
    let kind_adopt = match slot {
        ShopSlot::Card => EffectKind::CardAdopt { landing: false },
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
