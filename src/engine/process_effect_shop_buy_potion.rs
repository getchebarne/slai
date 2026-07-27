use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::engine::process_effect_shop_build::restock_potion;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::has_relic;

pub fn process_effect_shop_buy_potion(id_target: Option<usize>, state: &mut GameState) {
    // Sozu: no purchase happens at all — no gold spent, slot stays stocked
    if has_relic(&state.id_relics, RelicName::Sozu) {
        return;
    }

    // Find and remove the shop entry
    let id_potion = id_target.expect("ShopBuyPotion requires id_target");
    let Mode::Shop {
        shop_id_potions, ..
    } = &mut state.mode
    else {
        unreachable!("ShopBuyPotion outside Shop mode")
    };
    let idx = shop_id_potions
        .iter()
        .position(|&id| id == id_potion)
        .expect("bought potion is a shop entry");
    shop_id_potions.remove(idx);
    let price = state.entities[id_potion].price;

    // The Courier: the emptied slot restocks
    if has_relic(&state.id_relics, RelicName::TheCourier) {
        let mut id_potions_vec = std::mem::take(shop_id_potions);
        restock_potion(state, &mut id_potions_vec, idx);
        let Mode::Shop {
            shop_id_potions, ..
        } = &mut state.mode
        else {
            unreachable!("ShopBuyPotion outside Shop mode")
        };
        *shop_id_potions = id_potions_vec;
    }

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
