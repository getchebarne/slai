use crate::consts::MAX_SIZE_DECK;
use crate::effect::effect_direct;
use crate::effect::EffectKind;
use crate::events::card_is_upgradable;
use crate::game::GameState;
use crate::utils::shuffle;

pub fn process_effect_card_upgrade_random_in_deck(state: &mut GameState, count: u8) {
    let mut candidates: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut cand_n = 0;
    for &id in &state.id_deck {
        if card_is_upgradable(&state.entities[id]) {
            candidates[cand_n] = id;
            cand_n += 1;
        }
    }
    shuffle(&mut candidates[..cand_n], &mut state.rng);
    let n = (count as usize).min(cand_n);
    for &id_card in candidates[..n].iter().rev() {
        state
            .effect_queue
            .push_front(effect_direct(EffectKind::CardUpgrade, None, Some(id_card)));
    }
}
