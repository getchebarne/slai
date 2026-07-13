use rand::Rng;

use crate::consts::MAX_SIZE_HAND;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::utils::card_is_upgradable;

// Warped Tongs: upgrade a random upgradable hand card (combat copies, so combat-scoped)
pub fn process_effect_warped_tongs_proc(state: &mut GameState) {
    let mut ids = [0usize; MAX_SIZE_HAND];
    let mut n = 0;
    for &id in &state.id_hand {
        if card_is_upgradable(&state.entities[id]) {
            ids[n] = id;
            n += 1;
        }
    }
    if n == 0 {
        return;
    }
    let id_card = ids[state.rng.random_range(0..n)];
    state.effect_queue.push_front(Effect {
        kind: EffectKind::CardUpgrade,
        id_source: None,
        target: Target::Direct(Some(id_card)),
    });
}
