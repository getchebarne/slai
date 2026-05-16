use std::collections::VecDeque;

use rand::Rng;

use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::entity::Entity;
use crate::events::card_is_upgradable;
use crate::types::Phase;
use crate::utils::shuffle;

pub fn process_effect_card_upgrade_random_in_deck(
    count: u8,
    entities: &[Entity],
    id_deck: &[usize],
    rng: &mut impl Rng,
    effect_queue: &mut VecDeque<Effect>,
) -> Option<Phase> {
    let mut candidates: Vec<usize> = id_deck
        .iter()
        .copied()
        .filter(|&id| card_is_upgradable(&entities[id]))
        .collect();
    shuffle(&mut candidates, rng);
    let n = (count as usize).min(candidates.len());
    for &id_card in candidates[..n].iter().rev() {
        effect_queue.push_front(Effect::direct(EffectKind::CardUpgrade, None, Some(id_card)));
    }
    None
}
