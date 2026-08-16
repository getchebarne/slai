use rand::Rng;

use crate::consts::MAX_GOLD;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;
use crate::types::Focus;
use crate::types::RelicName;
use crate::utils::context_focus;
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
        _ => {
            unreachable!("GoldDelta only resolves Absolute, Relative, or Range")
        }
    };

    // Ectoplasm: gold can no longer be gained (after resolution, for RNG parity with the source game)
    if sign == DeltaSign::Gain && has_relic(&state.id_relics, RelicName::Ectoplasm) {
        return;
    }

    // Bloody Idol: any actual gold gain heals 5; Ectoplasm-blocked gains don't reach here
    if sign == DeltaSign::Gain && amount > 0 && has_relic(&state.id_relics, RelicName::BloodyIdol) {
        state.effect_queue.push_front(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Maw Bank deactivates the first time gold is spent at a shop (event costs don't count)
    if sign == DeltaSign::Loss
        && amount > 0
        && context_focus(state) == Focus::Shop
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
