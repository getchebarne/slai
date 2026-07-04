use crate::consts::MAX_SIZE_DECK;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::effect::HealthDeltaAmount;
use crate::game::GameState;
use crate::map::get_active_room_kind;
use crate::modifier::ModifierKind;
use crate::relics::RELIC_COUNTERS_PER_COMBAT;
use crate::relics::RELIC_COUNTERS_PER_TURN;
use crate::relics::iter_owned_relics;
use crate::types::CardKind;
use crate::types::DeltaSign;
use crate::types::MonsterKind;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::push_entity;
use crate::utils::shuffle;

pub fn process_effect_combat_start(state: &mut GameState) {
    state.this_combat_damage_instances_taken = 0;
    state.this_combat_escaped = false;
    state.this_turn_cards_played = 0;

    // Combat can end mid-turn, skipping the turn-end reset
    for &name in RELIC_COUNTERS_PER_TURN.iter().chain(RELIC_COUNTERS_PER_COMBAT) {
        if let Some(id) = state.id_relics[name as usize] {
            state.entities[id].relic_counter = 0;
        }
    }

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

    // Ancient Tea Set: primed at the last rest site; sip for 2 energy, then unprime
    if let Some(id) = state.id_relics[RelicName::AncientTeaSet as usize]
        && state.entities[id].relic_counter == 1
    {
        state.entities[id].relic_counter = 0;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::EnergyGain { amount: 2 },
            id_source: None,
            target: Target::Direct(None),
        });
    }

    // Preserved Insect: elites start at 75% HP; direct write skips damage triggers
    // (a HealthDelta would wake Lagavulin and decrement PlatedArmor)
    if state.id_relics[RelicName::PreservedInsect as usize].is_some()
        && matches!(
            get_active_room_kind(&state.id_rooms, state.location, &state.entities),
            Some(RoomKind::CombatElite)
        )
    {
        let id_monsters = state.id_monsters;
        for id in id_monsters.iter().flatten().copied() {
            let vitals = &mut state.entities[id].vitals;
            vitals.health = (vitals.health_max as u32 * 3 / 4) as u16;
        }
    }

    // Du-Vu Doll: +1 Strength per Curse in the master deck
    if state.id_relics[RelicName::DuVuDoll as usize].is_some() {
        let curses = state
            .id_deck
            .iter()
            .filter(|&&id| state.entities[id].card_kind == CardKind::Curse)
            .count();
        if curses > 0 {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::ModifierGain {
                    kind: ModifierKind::Strength,
                    stacks: curses as i16,
                },
                id_source: None,
                target: Target::Direct(Some(state.id_character)),
            });
        }
    }

    // Pantograph: boss fights open with a 25 HP heal
    if state.id_relics[RelicName::Pantograph as usize].is_some()
        && state
            .id_monsters
            .iter()
            .flatten()
            .any(|&id| state.entities[id].monster_kind == MonsterKind::Boss)
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::HealthDelta {
                sign: DeltaSign::Gain,
                amount: HealthDeltaAmount::Absolute(25),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // Sling of Courage: elite fights open with 2 Strength (eliteTrigger is elite-only)
    if state.id_relics[RelicName::SlingOfCourage as usize].is_some()
        && matches!(
            get_active_room_kind(&state.id_rooms, state.location, &state.entities),
            Some(RoomKind::CombatElite)
        )
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::ModifierGain {
                kind: ModifierKind::Strength,
                stacks: 2,
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
    use crate::game::GameState;
    use crate::game::create_game_state;
    use crate::modifier::ModifierKind;
    use crate::modifier::modifier_stacks;
    use crate::types::MonsterName;
    use crate::types::RelicName;
    use crate::utils::grant_relic;

    fn start_combat(state: &mut GameState, monster: MonsterName) {
        for kind in [
            EffectKind::MonsterSpawn { name: monster },
            EffectKind::CombatStart,
        ] {
            state.effect_queue.push_back(Effect {
                kind,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        process_effect_queue(state);
    }

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
    fn du_vu_doll_grants_strength_per_curse() {
        let mut state = create_game_state(0, 42, false);
        grant_relic(RelicName::DuVuDoll, &mut state.id_relics, &mut state.entities);
        for _ in 0..2 {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::CardAddToDeck {
                    card_name: crate::types::CardName::Regret,
                    upgraded: false,
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        process_effect_queue(&mut state);
        start_combat(&mut state, MonsterName::JawWorm);
        let mods = &state.entities[state.id_character].modifiers;
        assert_eq!(modifier_stacks(mods, ModifierKind::Strength), 2);
    }

    #[test]
    fn pantograph_heals_on_boss_combat() {
        let mut state = create_game_state(0, 42, false);
        grant_relic(RelicName::Pantograph, &mut state.id_relics, &mut state.entities);
        let id_character = state.id_character;
        state.entities[id_character].vitals.health -= 30;
        let hp_before = state.entities[id_character].vitals.health;
        start_combat(&mut state, MonsterName::TheGuardian);
        assert_eq!(state.entities[id_character].vitals.health, hp_before + 25);
        // Normal fights heal nothing
        let mut state = create_game_state(0, 42, false);
        grant_relic(RelicName::Pantograph, &mut state.id_relics, &mut state.entities);
        let id_character = state.id_character;
        state.entities[id_character].vitals.health -= 30;
        let hp_before = state.entities[id_character].vitals.health;
        start_combat(&mut state, MonsterName::JawWorm);
        assert_eq!(state.entities[id_character].vitals.health, hp_before);
    }

    #[test]
    fn preserved_insect_cuts_elite_hp_without_waking_lagavulin() {
        let mut state = create_game_state(0, 42, false);
        grant_relic(
            RelicName::PreservedInsect,
            &mut state.id_relics,
            &mut state.entities,
        );
        let x = (0..crate::consts::MAP_WIDTH)
            .find(|&x| state.id_rooms[0][x].is_some())
            .expect("row 0 has a room");
        let id_room = state.id_rooms[0][x].unwrap();
        state.entities[id_room].room_kind = crate::types::RoomKind::CombatElite;
        state.location = crate::game::Location::Overworld { y: 0, x };
        start_combat(&mut state, MonsterName::Lagavulin);
        let id_monster = state.id_monsters.iter().flatten().copied().next().unwrap();
        let monster = &state.entities[id_monster];
        assert_eq!(
            monster.vitals.health,
            (monster.vitals.health_max as u32 * 3 / 4) as u16
        );
        // The direct write skips damage triggers: Lagavulin stays asleep
        assert!(crate::modifier::modifier_has(
            &monster.modifiers,
            crate::modifier::ModifierKind::Asleep
        ));
    }

    #[test]
    fn sling_of_courage_arms_elite_fights() {
        let mut state = create_game_state(0, 42, false);
        grant_relic(
            RelicName::SlingOfCourage,
            &mut state.id_relics,
            &mut state.entities,
        );
        let x = (0..crate::consts::MAP_WIDTH)
            .find(|&x| state.id_rooms[0][x].is_some())
            .expect("row 0 has a room");
        let id_room = state.id_rooms[0][x].unwrap();
        state.entities[id_room].room_kind = crate::types::RoomKind::CombatElite;
        state.location = crate::game::Location::Overworld { y: 0, x };
        start_combat(&mut state, MonsterName::JawWorm);
        let mods = &state.entities[state.id_character].modifiers;
        assert_eq!(modifier_stacks(mods, ModifierKind::Strength), 2);
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
