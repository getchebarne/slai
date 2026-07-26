use rand::Rng;

use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::events::EventKind;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Mode;
use crate::types::RelicName;

pub fn process_effect_gold_delta(state: &mut GameState, sign: DeltaSign, amount: Amount) {
    let amount = match amount {
        Amount::Absolute(a) => a,
        Amount::Range { min, max } => state.rng.random_range(min..=max),
        Amount::EventGoldAsk => {
            let Mode::Event {
                kind: EventKind::WeMeetAgain { gold_ask, .. },
                ..
            } = &state.mode
            else {
                unreachable!("EventGoldAsk outside We Meet Again")
            };
            gold_ask.expect("EventGoldAsk without a rolled ask")
        }
        _ => {
            unreachable!("GoldDelta only resolves Absolute, Range, or EventGoldAsk")
        }
    };

    // Maw Bank deactivates the first time gold is spent at a shop (event costs don't count)
    if sign == DeltaSign::Loss
        && amount > 0
        && matches!(state.mode, Mode::Shop { .. })
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
