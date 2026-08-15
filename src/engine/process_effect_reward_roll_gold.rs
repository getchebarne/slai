use rand::Rng;

use crate::effect::Amount;
use crate::game::GameState;
use crate::types::RelicName;
use crate::types::reward_ensure;
use crate::utils::has_relic;

pub fn process_effect_reward_roll_gold(state: &mut GameState, amount: Amount) {
    let mut rolled = match amount {
        Amount::Absolute(amount) => amount,
        Amount::Range { min, max } => state.rng.random_range(min..=max),
        _ => unreachable!("Reward gold only resolves Absolute or Range"),
    };

    // Golden Idol: 25% bonus rounded half-up on combat rewards only
    if has_relic(&state.id_relics, RelicName::GoldenIdol) {
        rolled += (rolled + 2) / 4;
    }

    reward_ensure(&mut state.reward);
    state.reward.gold = Some(rolled);
}
