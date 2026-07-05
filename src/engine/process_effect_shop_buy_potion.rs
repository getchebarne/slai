use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::GoldDeltaKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::grant_potion;
use crate::types::DeltaSign;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_shop_buy_potion(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("ShopBuyPotion requires id_target");
    let idx = state
        .shop_id_potions
        .iter()
        .position(|&id| id == id_potion)
        .expect("bought potion is a shop entry");
    state.shop_id_potions.remove(idx);
    let price = state.shop_potion_prices.remove(idx);
    let name = state.entities[id_potion].potion_name;

    grant_potion(&mut state.id_potions, state.potion_slots_max, &mut state.entities, name)
        .expect("legal_actions guarantees the belt has a free slot");

    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            kind: GoldDeltaKind::Fixed(price),
        },
        id_source: None,
        target: Target::Direct(None),
    });
    flush_effects_from_buf_to_queue_front(state);
}
