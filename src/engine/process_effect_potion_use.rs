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
    if state.id_relics[RelicName::ToyOrnithopter as usize].is_some() {
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

#[cfg(test)]
mod tests {
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::game::create_game_state;
    use crate::potions::get_potion;
    use crate::types::PotionName;
    use crate::types::RelicName;
    use crate::engine::test_support::grant_relic;
    use crate::utils::push_entity;

    #[test]
    fn toy_ornithopter_heals_on_potion_use() {
        let mut state = create_game_state(0, 42, false);
        grant_relic(
            RelicName::ToyOrnithopter,
            &mut state.id_relics,
            &mut state.entities,
        );
        let id_character = state.id_character;
        state.entities[id_character].vitals.health -= 10;
        let hp_before = state.entities[id_character].vitals.health;
        let id_potion = push_entity(&mut state.entities, get_potion(PotionName::BlockPotion));
        state.id_potions[0] = Some(id_potion);
        state.effect_queue.push_back(Effect {
            kind: EffectKind::PotionUse,
            id_source: None,
            target: Target::Direct(Some(id_potion)),
        });
        process_effect_queue(&mut state);
        assert_eq!(state.entities[id_character].vitals.health, hp_before + 5);
    }
}
