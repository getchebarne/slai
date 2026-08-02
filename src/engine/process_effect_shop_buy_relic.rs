use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::shop::restock_relic;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::mode_top_mut;

pub fn process_effect_shop_buy_relic(id_target: Option<usize>, state: &mut GameState) {
    // Find and remove the shop entry
    let id_relic = id_target.expect("ShopBuyRelic requires id_target");
    let Mode::Shop {
        shop_id_cards,
        shop_id_relics,
        shop_id_potions,
        shop_purge_cost,
        ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("ShopBuyRelic outside Shop mode")
    };

    // Take Relic
    let idx = shop_id_relics
        .iter()
        .position(|&id| id == id_relic)
        .expect("Bought Relic is a shop entry");
    shop_id_relics.remove(idx);

    // Snapshot bought price and name
    let price_bought = state.entities[id_relic].price;
    let name_bought = state.entities[id_relic].relic_name;

    // Membership Card bought mid-shop: retro-discount the remaining stock and purge
    if name_bought == RelicName::MembershipCard {
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

    // The Courier: the emptied slot restocks
    if name_bought == RelicName::TheCourier || has_relic(&state.id_relics, RelicName::TheCourier) {
        // Restock as if the sale settled: priced with the bought Relic, never re-offering it
        let mut id_relics_settled = state.id_relics;
        id_relics_settled[name_bought as usize] = Some(id_relic);
        restock_relic(
            &mut state.entities,
            &mut state.rng,
            &id_relics_settled,
            shop_id_relics,
            idx,
        );
    }

    // Charge gold and grant the Relic
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
        kind: EffectKind::RelicAdopt,
        id_source: None,
        target: Target::Direct(Some(id_relic)),
    });
    flush_effects_from_buf_to_queue_front(state);
}
