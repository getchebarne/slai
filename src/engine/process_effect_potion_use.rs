use crate::utils::has_relic;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::potions::remove_potion;
use crate::types::DeltaSign;
use crate::types::RelicName;

pub fn process_effect_potion_use(id_target: Option<usize>, state: &mut GameState) {
    let id_potion = id_target.expect("PotionUse requires id_target");
    // Consume the potion from its belt slot before its effects run
    remove_potion(&mut state.id_potions, id_potion);
    let potion = &state.entities[id_potion];
    for effect in potion.potion_effects.iter().rev() {
        state.effect_queue.push_front(Effect {
            id_source: Some(id_potion),
            ..*effect
        });
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
