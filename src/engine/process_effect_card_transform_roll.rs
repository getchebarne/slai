use rand::Rng;

use crate::cards::POOL_COMMON_GREEN_CARD;
use crate::cards::POOL_RARE_GREEN_CARD;
use crate::cards::POOL_UNCOMMON_GREEN_CARD;
use crate::consts::CARD_REWARD_ROLL_CHANCE_RARE;
use crate::consts::CARD_REWARD_ROLL_CHANCE_UNCOMMON;
use crate::effect::effect_direct;
use crate::effect::EffectKind;
use crate::game::GameState;

pub fn process_effect_card_transform_roll(state: &mut GameState) {
    let roll = state.rng.random_range(0..99);
    let pool = if roll < CARD_REWARD_ROLL_CHANCE_RARE {
        POOL_RARE_GREEN_CARD
    } else if roll < CARD_REWARD_ROLL_CHANCE_UNCOMMON {
        POOL_UNCOMMON_GREEN_CARD
    } else {
        POOL_COMMON_GREEN_CARD
    };
    let card_name = pool[state.rng.random_range(0..pool.len())];
    state.effect_queue.push_front(effect_direct(
        EffectKind::CardAddToDeck {
            card_name,
            upgraded: false,
        },
        None,
        None,
    ));
}
