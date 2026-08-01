use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::remove_potion;
use crate::types::DeltaSign;
use crate::types::RelicName;
use crate::utils::has_relic;

pub fn process_effect_potion_use(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("PotionUse requires id_target");
    // Consume the potion from its belt slot before its effects run
    remove_potion(&mut state.id_potions, id_potion);
    // Sacred Bark: potion effects double; discover potions fall through untouched
    let sacred_bark = has_relic(&state.id_relics, RelicName::SacredBark);
    let potion = &state.entities[id_potion];
    for effect in potion.potion_effects.iter().rev() {
        let mut effect = Effect {
            id_source: Some(id_potion),
            ..*effect
        };
        if sacred_bark {
            match &mut effect.kind {
                EffectKind::BlockGain { amount }
                | EffectKind::DamagePhysical { amount }
                | EffectKind::EnergyDelta { amount, .. } => *amount *= 2,
                EffectKind::ModifierGain { stacks, .. } => *stacks *= 2,
                EffectKind::CardDraw { count } => *count *= 2,
                EffectKind::HealthDelta { amount, .. }
                | EffectKind::MaxHealthDelta { amount, .. } => {
                    if let Amount::Absolute(a) = amount {
                        *a *= 2;
                    }
                }
                _ => {}
            }
        }
        state.effect_queue.push_front(effect);
    }

    // Toy Ornithopter: any potion use heals 5, in or out of combat
    if has_relic(&state.id_relics, RelicName::ToyOrnithopter) {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }
}
