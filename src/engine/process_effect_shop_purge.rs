use crate::consts::SHOP_PURGE_COST_INCREMENT;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::game::Location;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::utils::flush_effects_from_buf_to_queue_front;

pub fn process_effect_shop_purge(id_target: Option<usize>, state: &mut GameState) {
    // Charge gold and purge the picked card
    let id_card = id_target.expect("ShopPurge requires id_target");
    let Mode::Shop {
        shop_purge_cost, ..
    } = &state.mode
    else {
        unreachable!("ShopPurge outside Shop mode")
    };
    let cost = *shop_purge_cost;

    state.effect_buf.clear();
    state.effect_buf.push(Effect {
        kind: EffectKind::GoldDelta {
            sign: DeltaSign::Loss,
            amount: Amount::Absolute(cost),
        },
        id_source: None,
        target: Target::Direct(None),
    });
    state.effect_buf.push(Effect {
        kind: EffectKind::CardPurge,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
    flush_effects_from_buf_to_queue_front(state);

    // Ramps for the rest of the run; the next shop build reads the new value
    state.shop_purge_cost_run += SHOP_PURGE_COST_INCREMENT;

    // A shop's card removal can be used once per visit
    if let Location::Overworld { y, x } = state.location
        && let Some(id_room) = state.id_rooms[y][x]
    {
        state.entities[id_room].room_shop_purged = true;
    }
}
