use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::shop::restock_potion;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;
use crate::utils::mode_top_mut;

pub fn process_effect_shop_buy_potion(id_target: Option<usize>, state: &mut GameState) {
    // Sozu: no purchase happens at all — no gold spent, slot stays stocked
    if has_relic(&state.id_relics, RelicName::Sozu) {
        return;
    }

    // Find and remove the shop entry
    let id_potion = id_target.expect("ShopBuyPotion requires id_target");
    let Mode::Shop {
        shop_id_potions, ..
    } = mode_top_mut(&mut state.mode_stack)
    else {
        unreachable!("ShopBuyPotion outside Shop mode")
    };

    // Take Potion
    let idx = shop_id_potions
        .iter()
        .position(|&id| id == id_potion)
        .expect("Bought Potion is a shop entry");
    shop_id_potions.remove(idx);

    // Snapshot bought price
    let price = state.entities[id_potion].price;

    // The Courier: the emptied slot restocks
    if has_relic(&state.id_relics, RelicName::TheCourier) {
        restock_potion(
            &mut state.entities,
            &mut state.rng,
            &state.id_relics,
            shop_id_potions,
            idx,
        );
    }

    // Charge gold and slot the Potion into the belt
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
