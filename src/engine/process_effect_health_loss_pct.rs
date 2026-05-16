use std::collections::VecDeque;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::entity::Entity;
use crate::types::Phase;

pub fn process_effect_health_loss_pct(
    character: &Entity,
    id_character: usize,
    numer: u8,
    denom: u8,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let amount = ((character.vitals.health_max as u32 * numer as u32) / denom as u32).max(1) as u16;
    effect_queue.push_front(Effect::direct(
        EffectKind::HealthLoss { amount },
        None,
        Some(id_character),
    ));
    None
}
