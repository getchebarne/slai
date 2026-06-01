use rand::Rng;

use crate::consts::MAX_GOLD;
use crate::effect::GoldDeltaKind;
use crate::game::GameState;
use crate::types::DeltaSign;

pub fn process_effect_gold_delta(state: &mut GameState, sign: DeltaSign, kind: GoldDeltaKind) {
    let amount = match kind {
        GoldDeltaKind::Fixed(a) => a,
        GoldDeltaKind::Range { min, max } => state.rng.random_range(min..=max),
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
