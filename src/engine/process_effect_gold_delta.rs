use rand::Rng;

use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::game::GameState;
use crate::types::DeltaSign;

pub fn process_effect_gold_delta(state: &mut GameState, sign: DeltaSign, amount: Amount) {
    let amount = match amount {
        Amount::Absolute(a) => a,
        Amount::Relative {
            numerator,
            denominator,
        } => {
            let gold = state.entities[state.id_character].character_gold;
            ((gold as u32 * numerator as u32) / denominator as u32) as u16
        }
        Amount::Range { min, max } => state.rng.random_range(min..=max),
    };
    let character = &mut state.entities[state.id_character];
    character.character_gold = match sign {
        DeltaSign::Gain => character
            .character_gold
            .saturating_add(amount)
            .min(MAX_GOLD),
        DeltaSign::Loss => character.character_gold.saturating_sub(amount),
    };
}
