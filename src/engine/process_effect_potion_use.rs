use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
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
    let sacred_bark = has_relic(&state.id_relics, RelicName::SacredBark);
    let potion = &state.entities[id_potion];

    // Push the Potions's on-use effects
    for effect in potion.potion_effects.iter().rev() {
        let mut effect = Effect {
            id_source: Some(id_potion), // Stamp the Potion's ID
            ..*effect
        };

        // Sacred Bark: potion effects double
        let mut repeat = false;
        if sacred_bark {
            match &mut effect.kind {
                // Stacks (Strength, Poison, Regeneration, Speed, ...)
                EffectKind::ModifierGain { stacks, .. } => *stacks *= 2,

                // Card count (Swift, Snecko Oil; Cunning's Shivs)
                EffectKind::CardDraw { count } | EffectKind::CardAdd { count, .. } => *count *= 2,

                // Intensity (Block, Fire, Explosive, Energy)
                EffectKind::BlockGain { amount }
                | EffectKind::DamagePhysical { amount }
                | EffectKind::EnergyDelta { amount, .. } => *amount *= 2,

                // Health (Fruit Juice); Relative amounts have no potency to scale
                EffectKind::HealthDelta { amount, .. }
                | EffectKind::MaxHealthDelta { amount, .. } => {
                    if let Amount::Absolute(a) = amount {
                        *a *= 2;
                    }
                }

                // Liquid Memories: potency doubles to two picks
                EffectKind::CardMove { .. } => {
                    if let Target::Resolve {
                        selection_kind: SelectionKind::Input { count },
                        ..
                    } = &mut effect.target
                    {
                        *count *= 2;
                    }
                }

                // Distilled Chaos: one play per effect, so the potency doubles by repeating
                EffectKind::CardPlayFromDrawTop => repeat = true,

                // No potency: Blessing of the Forge, Smoke Bomb, Gambler's Brew, Entropic
                // Brew, Snecko Oil's randomize
                _ => {}
            }
        }
        state.effect_queue.push_front(effect);
        if repeat {
            state.effect_queue.push_front(effect);
        }
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
