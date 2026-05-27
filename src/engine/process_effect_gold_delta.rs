use rand::Rng;

use crate::consts::MAX_GOLD;
use crate::effect::GoldDeltaKind;
use crate::effect::GoldDeltaSign;
use crate::game::GameState;

pub fn process_effect_gold_delta(state: &mut GameState, sign: GoldDeltaSign, kind: GoldDeltaKind) {
    let amount = match kind {
        GoldDeltaKind::Fixed(a) | GoldDeltaKind::PreRolled(a) => a,
        GoldDeltaKind::Range { min, max } => state.rng.random_range(min..=max),
    };
    let character = &mut state.entities[state.id_character];
    character.character_gold = match sign {
        GoldDeltaSign::Gain => character
            .character_gold
            .saturating_add(amount)
            .min(MAX_GOLD),
        GoldDeltaSign::Loss => character.character_gold.saturating_sub(amount),
    };
}
