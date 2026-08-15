use rand::Rng;

use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::events::EventKind;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Frame;
use crate::types::RelicName;
use crate::utils::frame_top;
use crate::utils::has_relic;

pub fn process_effect_gold_delta(state: &mut GameState, sign: DeltaSign, amount: Amount) {
    let amount = match amount {
        Amount::Absolute(a) => a,
        Amount::Range { min, max } => state.rng.random_range(min..=max),
        Amount::Relative {
            numerator,
            denominator,
        } => {
            let gold = state.entities[state.id_character].character_gold;
            (gold as u32 * numerator as u32 / denominator as u32) as u16
        }
        Amount::EventGoldAsk => {
            let Frame::Event {
                kind: EventKind::WeMeetAgain { gold_ask, .. },
                ..
            } = frame_top(&state.frame_stack)
            else {
                unreachable!("EventGoldAsk outside We Meet Again")
            };
            gold_ask.expect("EventGoldAsk without a rolled ask")
        }
        _ => {
            unreachable!("GoldDelta only resolves Absolute, Range, or EventGoldAsk")
        }
    };

    // Ectoplasm: gold can no longer be gained (after resolution, for RNG parity with the source game)
    if sign == DeltaSign::Gain && has_relic(&state.id_relics, RelicName::Ectoplasm) {
        return;
    }

    // Maw Bank deactivates the first time gold is spent at a shop (event costs don't count)
    if sign == DeltaSign::Loss
        && amount > 0
        && matches!(frame_top(&state.frame_stack), Frame::Shop { .. })
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
