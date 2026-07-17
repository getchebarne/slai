use rand::Rng;

use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::types::Screen;

pub fn process_effect_gold_delta(state: &mut GameState, sign: DeltaSign, amount: Amount) {
    let amount = match amount {
        Amount::Absolute(a) => a,
        Amount::Range { min, max } => state.rng.random_range(min..=max),
        Amount::EventRoll { idx } => {
            let id_event = state.id_event.expect("EventRoll outside an event");
            state.entities[id_event].event_rolls[idx as usize]
        }
        _ => {
            unreachable!("GoldDelta only resolves Absolute, Range, or EventRoll")
        }
    };

    // Maw Bank deactivates the first time gold is spent at a shop (event costs don't count)
    if sign == DeltaSign::Loss
        && amount > 0
        && matches!(state.screen, Screen::Shop)
        && let Some(id) = state.id_relics[RelicName::MawBank as usize]
    {
        state.entities[id].relic_used_up = true;
    }

    // Apply delta
    let character = &mut state.entities[state.id_character];
    character.character_gold = match sign {
        DeltaSign::Gain => character
            .character_gold
            .saturating_add(amount)
            .min(MAX_GOLD),
        DeltaSign::Loss => character.character_gold.saturating_sub(amount),
    };
}
