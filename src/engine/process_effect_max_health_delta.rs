use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::types::DeltaSign;

pub fn process_effect_max_health_delta(
    id_target: Option<usize>,
    state: &mut GameState,
    sign: DeltaSign,
    amount: Amount,
) {
    let id_target = id_target.expect("MaxHealthDelta requires id_target");

    // Resolve amount
    let amount = match amount {
        Amount::Absolute(a) => a,
        Amount::Relative {
            numerator,
            denominator,
        } => {
            let health_max = state.entities[id_target].vitals.health_max;
            // f32 mirrors the source's (int)(maxHP * fraction) float truncation
            let raw = (health_max as f32 * (numerator as f32 / denominator as f32)) as u32;
            match sign {
                DeltaSign::Loss => raw.max(1) as u16,
                DeltaSign::Gain => raw as u16,
            }
        }
        _ => {
            unreachable!("MaxHealthDelta only resolves Absolute or Relative")
        }
    };

    // Apply amount
    match sign {
        DeltaSign::Gain => {
            let vitals = &mut state.entities[id_target].vitals;
            vitals.health_max = vitals.health_max.saturating_add(amount);
            // The source's increaseMaxHp also heals by the same amount
            state.effect_queue.push_front(Effect {
                kind: EffectKind::HealthDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(amount),
                },
                id_source: None,
                target: Target::Direct(Some(id_target)),
            });
        }
        DeltaSign::Loss => {
            let vitals = &mut state.entities[id_target].vitals;
            vitals.health_max = vitals.health_max.saturating_sub(amount).max(1);
            vitals.health = vitals.health.min(vitals.health_max);
        }
    }
}
