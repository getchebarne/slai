use crate::consts::MAX_SIZE_DECK;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::game::GameState;
use crate::relics::iter_owned_relics;
use crate::utils::push_entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(state: &mut GameState) {
    state.this_combat_damage_instances_taken = 0;
    state.this_combat_escaped = false;
    state.this_turn_cards_played = 0;

    // Innate cards sit on top of the draw pile, ahead of the shuffled rest
    let mut other_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut other_n: usize = 0;
    let mut innate_ids: [usize; MAX_SIZE_DECK] = [0; MAX_SIZE_DECK];
    let mut innate_n: usize = 0;

    for i in 0..state.id_deck.len() {
        let id_card_src = state.id_deck[i];
        let card = state.entities[id_card_src];
        let id_card = push_entity(&mut state.entities, card);
        if card.card_innate {
            innate_ids[innate_n] = id_card;
            innate_n += 1;
        } else {
            other_ids[other_n] = id_card;
            other_n += 1;
        }
    }

    shuffle(&mut other_ids[..other_n], &mut state.rng);

    state.id_pile_draw.clear();
    for &id in &other_ids[..other_n] {
        state.id_pile_draw.push(id);
    }
    for &id in &innate_ids[..innate_n] {
        state.id_pile_draw.push(id);
    }

    state.id_picked_monster = None;

    // Monster MoveUpdates already queued at MonsterSpawn; queue character TurnStart
    state.effect_queue.push_front(Effect {
        kind: EffectKind::TurnStart,
        id_source: None,
        target: Target::Direct(Some(state.id_character)),
    });

    for (_name, id_relic) in iter_owned_relics(&state.id_relics) {
        for &eff in state.entities[id_relic].relic_effects_on_combat_start {
            state.effect_queue.push_back(eff);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::effect::Effect;
    use crate::effect::EffectKind;
    use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::game::GameState;
    use crate::game::create_game_state;
    use crate::modifier::ModifierKind;
    use crate::modifier::modifier_stacks;
    use crate::types::MonsterName;
    use crate::types::RelicName;
    use crate::utils::grant_relic;

    fn combat_with_relic(relic: RelicName) -> GameState {
        let mut state = create_game_state(0, 42, false);
        grant_relic(relic, &mut state.id_relics, &mut state.entities);
        for kind in [
            EffectKind::MonsterSpawn {
                name: MonsterName::JawWorm,
            },
            EffectKind::CombatStart,
        ] {
            state.effect_queue.push_back(Effect {
                kind,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        process_effect_queue(&mut state);
        state
    }

    #[test]
    fn lantern_adds_energy_on_first_turn() {
        let state = combat_with_relic(RelicName::Lantern);
        assert_eq!(state.energy.energy_current, 4);
        assert_eq!(state.energy.energy_max, 3);
    }

    #[test]
    fn clockwork_souvenir_grants_artifact() {
        let state = combat_with_relic(RelicName::ClockworkSouvenir);
        let mods = &state.entities[state.id_character].modifiers;
        assert_eq!(modifier_stacks(mods, ModifierKind::Artifact), 1);
    }

    #[test]
    fn gremlin_visage_weakens_character() {
        let state = combat_with_relic(RelicName::GremlinVisage);
        let mods = &state.entities[state.id_character].modifiers;
        assert_eq!(modifier_stacks(mods, ModifierKind::Weak), 1);
    }

    #[test]
    fn red_mask_weakens_all_monsters() {
        let state = combat_with_relic(RelicName::RedMask);
        let weakened: Vec<usize> = state
            .id_monsters
            .iter()
            .flatten()
            .filter(|&&id| modifier_stacks(&state.entities[id].modifiers, ModifierKind::Weak) == 1)
            .copied()
            .collect();
        assert_eq!(weakened.len(), 1);
    }
}
