use crate::effect::effect_direct;
use crate::effect::EffectKind;
use crate::game::GameState;

pub fn process_effect_health_loss_pct(state: &mut GameState, numer: u8, denom: u8) {
    let id_character = state.id_character;
    let character = &state.entities[id_character];
    let amount = ((character.vitals.health_max as u32 * numer as u32) / denom as u32).max(1) as u16;
    state.effect_queue.push_front(effect_direct(
        EffectKind::HealthLoss { amount },
        None,
        Some(id_character),
    ));
}
