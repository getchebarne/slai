use rand::Rng;

use crate::consts::CHEST_SMALL_PCT;
use crate::consts::CHEST_SMALL_PLUS_MEDIUM_PCT;
use crate::consts::EVENT_SHRINE_CHANCE;
use crate::consts::UNKNOWN_CHANCE_BASE_MONSTER;
use crate::consts::UNKNOWN_CHANCE_BASE_SHOP;
use crate::consts::UNKNOWN_CHANCE_BASE_TREASURE;
use crate::effect::Amount;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::spawn_event;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::map::room_at_mut;
use crate::types::ChestKind;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::MonsterEncounter;
use crate::types::MonsterName;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::types::Screen;
use crate::utils::flush_effects_from_buf_to_queue_front;
use crate::utils::push_entity;
use crate::utils::shuffle;

pub fn process_effect_room_enter(state: &mut GameState) {
    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities).unwrap();
    state.effect_buf.clear();

    // Maw Bank: 12 gold on every room entry until deactivated by spending at a shop.
    // Straight to effect_queue: effect_buf is reserved for the combat-spawn path below
    if let Some(id) = state.id_relics[RelicName::MawBank as usize]
        && !state.entities[id].relic_used_up
    {
        state.effect_queue.push_back(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Gain,
                amount: Amount::Absolute(12),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
    }

    // A "?" (Unknown) node resolves into a concrete kind on entry via drifting odds
    let resolved = if room_kind == RoomKind::Unknown {
        roll_unknown_room(state)
    } else {
        room_kind
    };

    match resolved {
        RoomKind::CombatBoss => {
            let encounter = state.encounter_boss;
            spawn_encounter_monsters(encounter, &mut state.effect_buf, &mut state.rng);
        }
        RoomKind::CombatMonster => {
            let encounter = state.encounter_pool_normal.remove(0);
            spawn_encounter_monsters(encounter, &mut state.effect_buf, &mut state.rng);
        }
        RoomKind::CombatElite => {
            let encounter = state.encounter_pool_elite.remove(0);
            spawn_encounter_monsters(encounter, &mut state.effect_buf, &mut state.rng);
        }
        RoomKind::RestSite => {
            state.screen = Screen::RestSite;
            // Eternal Feather: 3 HP per 5 deck cards on arrival
            if state.id_relics[RelicName::EternalFeather as usize].is_some() {
                let heal = (state.id_deck.len() / 5) * 3;
                if heal > 0 {
                    state.effect_queue.push_back(Effect {
                        kind: EffectKind::HealthDelta {
                            sign: DeltaSign::Gain,
                            amount: Amount::Absolute(heal as u16),
                        },
                        id_source: None,
                        target: Target::Direct(Some(state.id_character)),
                    });
                }
            }
            // Ancient Tea Set: prime for the next combat
            if let Some(id) = state.id_relics[RelicName::AncientTeaSet as usize] {
                state.entities[id].relic_counter = 1;
            }
        }
        RoomKind::Treasure => {
            let Location::Overworld { y, x } = state.location else {
                unreachable!("RoomEnter on Treasure outside Overworld");
            };
            let room = room_at_mut(&state.id_rooms, &mut state.entities, y, x)
                .expect("Treasure room missing");
            let roll = state.rng.random_range(0..100) as u8;
            room.room_chest_kind = Some(if roll < CHEST_SMALL_PCT {
                ChestKind::Small
            } else if roll < CHEST_SMALL_PLUS_MEDIUM_PCT {
                ChestKind::Medium
            } else {
                ChestKind::Large
            });
            state.screen = Screen::Chest;
        }
        RoomKind::EventRoom => {
            if let Some(id_event) = spawn_random_event(state) {
                state.screen = Screen::Event;
                state.id_event = Some(id_event);
                return;
            }
        }
        RoomKind::Shop => {
            state.screen = Screen::Shop;
            if state.id_relics[RelicName::MealTicket as usize].is_some() {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::HealthDelta {
                        sign: DeltaSign::Gain,
                        amount: Amount::Absolute(15),
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
            state.effect_queue.push_front(Effect {
                kind: EffectKind::ShopBuild,
                id_source: None,
                target: Target::Direct(None),
            });
        }
        RoomKind::Unknown => {
            unreachable!("Unknown is resolved into a concrete kind before dispatch")
        }
    }

    if !state.effect_buf.is_empty() {
        state.effect_buf.push(Effect {
            kind: EffectKind::CombatStart,
            id_source: None,
            target: Target::Direct(None),
        });
        flush_effects_from_buf_to_queue_front(state);
    }
}

// Resolve a "?" room into a concrete kind, then drift the running tallies
fn roll_unknown_room(state: &mut GameState) -> RoomKind {
    // Tiny Chest: every 4th ? room is forced Treasure; drift still runs as if rolled
    let forced_treasure = if let Some(id) = state.id_relics[RelicName::TinyChest as usize] {
        let counter = &mut state.entities[id].relic_counter;
        *counter += 1;
        if *counter >= 4 {
            *counter = 0;
            true
        } else {
            false
        }
    } else {
        false
    };

    let mut resolved = if forced_treasure {
        RoomKind::Treasure
    } else {
        let idx = state.rng.random_range(0..100) as i32;
        let chance_monster = (state.unknown_chance_monster * 100.0) as i32;
        let chance_shop = (state.unknown_chance_shop * 100.0) as i32;
        let chance_treasure = (state.unknown_chance_treasure * 100.0) as i32;

        if idx < chance_monster {
            RoomKind::CombatMonster
        } else if idx < chance_monster + chance_shop {
            RoomKind::Shop
        } else if idx < chance_monster + chance_shop + chance_treasure {
            RoomKind::Treasure
        } else {
            RoomKind::EventRoom
        }
    };

    // Drift: the chosen type resets to base, every other type accumulates by its base
    state.unknown_chance_monster = if resolved == RoomKind::CombatMonster {
        UNKNOWN_CHANCE_BASE_MONSTER
    } else {
        state.unknown_chance_monster + UNKNOWN_CHANCE_BASE_MONSTER
    };
    state.unknown_chance_shop = if resolved == RoomKind::Shop {
        UNKNOWN_CHANCE_BASE_SHOP
    } else {
        state.unknown_chance_shop + UNKNOWN_CHANCE_BASE_SHOP
    };
    state.unknown_chance_treasure = if resolved == RoomKind::Treasure {
        UNKNOWN_CHANCE_BASE_TREASURE
    } else {
        state.unknown_chance_treasure + UNKNOWN_CHANCE_BASE_TREASURE
    };

    // Juzu Bracelet: a monster resolution becomes an event (after the drift settles)
    if resolved == RoomKind::CombatMonster
        && state.id_relics[RelicName::JuzuBracelet as usize].is_some()
    {
        resolved = RoomKind::EventRoom;
    }

    resolved
}

// 25% shrine pool, else event pool; an exhausted pool falls back to the other,
// both empty -> no event and the room is a no-op
fn spawn_random_event(state: &mut GameState) -> Option<usize> {
    let name = if state.rng.random_range(0.0..1.0f32) < EVENT_SHRINE_CHANCE {
        draw_shrine(state).or_else(|| draw_event(state))
    } else {
        draw_event(state).or_else(|| draw_shrine(state))
    }?;
    let event = spawn_event(name, state.ascension, &mut state.rng);
    Some(push_entity(&mut state.entities, event))
}

fn draw_event(state: &mut GameState) -> Option<EventName> {
    // The Cleric only spawns with gold for its cheapest option; it stays pooled otherwise
    let gold = state.entities[state.id_character].character_gold;
    let eligible: Vec<usize> = state
        .pool_events
        .iter()
        .enumerate()
        .filter(|&(_, &name)| name != EventName::TheCleric || gold >= 35)
        .map(|(i, _)| i)
        .collect();
    if eligible.is_empty() {
        return None;
    }
    let idx = eligible[state.rng.random_range(0..eligible.len())];
    Some(state.pool_events.swap_remove(idx))
}

fn draw_shrine(state: &mut GameState) -> Option<EventName> {
    if state.pool_shrines.is_empty() {
        return None;
    }
    let idx = state.rng.random_range(0..state.pool_shrines.len());
    Some(state.pool_shrines.swap_remove(idx))
}

fn pick_louse(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::LouseNormal
    } else {
        MonsterName::LouseDefensive
    }
}

fn pick_slaver(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::SlaverRed
    } else {
        MonsterName::SlaverBlue
    }
}

fn pick_wildlife_weak(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => pick_louse(rng),
        1 => MonsterName::SlimeSpikeMedium,
        2 => MonsterName::SlimeAcidMedium,
        _ => unreachable!(),
    }
}

fn pick_wildlife_strong(rng: &mut impl Rng) -> MonsterName {
    if rng.random_bool(0.5) {
        MonsterName::FungiBeast
    } else {
        MonsterName::JawWorm
    }
}

fn pick_humanoid_strong(rng: &mut impl Rng) -> MonsterName {
    match rng.random_range(0..3) {
        0 => MonsterName::Cultist,
        1 => pick_slaver(rng),
        2 => MonsterName::Looter,
        _ => unreachable!(),
    }
}

fn push_monster_spawn(effects: &mut Vec<Effect>, name: MonsterName) {
    effects.push(Effect {
        kind: EffectKind::MonsterSpawn { name },
        id_source: None,
        target: Target::Direct(None),
    });
}

fn spawn_encounter_monsters(
    encounter: MonsterEncounter,
    effects: &mut Vec<Effect>,
    rng: &mut impl Rng,
) {
    match encounter {
        MonsterEncounter::Cultist => push_monster_spawn(effects, MonsterName::Cultist),
        MonsterEncounter::JawWorm => push_monster_spawn(effects, MonsterName::JawWorm),
        MonsterEncounter::TwoLouse => {
            for _ in 0..2 {
                push_monster_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::SmallSlimes => {
            let (small, medium) = if rng.random_bool(0.5) {
                (MonsterName::SlimeSpikeSmall, MonsterName::SlimeAcidMedium)
            } else {
                (MonsterName::SlimeAcidSmall, MonsterName::SlimeSpikeMedium)
            };
            push_monster_spawn(effects, small);
            push_monster_spawn(effects, medium);
        }
        MonsterEncounter::BlueSlaver => push_monster_spawn(effects, MonsterName::SlaverBlue),
        MonsterEncounter::RedSlaver => push_monster_spawn(effects, MonsterName::SlaverRed),
        MonsterEncounter::Looter => push_monster_spawn(effects, MonsterName::Looter),
        MonsterEncounter::TwoFungiBeasts => {
            push_monster_spawn(effects, MonsterName::FungiBeast);
            push_monster_spawn(effects, MonsterName::FungiBeast);
        }
        MonsterEncounter::ThreeLouse => {
            for _ in 0..3 {
                push_monster_spawn(effects, pick_louse(rng));
            }
        }
        MonsterEncounter::LargeSlime => {
            let name = if rng.random_bool(0.5) {
                MonsterName::SlimeAcidLarge
            } else {
                MonsterName::SlimeSpikeLarge
            };
            push_monster_spawn(effects, name);
        }
        MonsterEncounter::LotsOfSlimes => {
            let mut pool = [
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeSpikeSmall,
                MonsterName::SlimeAcidSmall,
                MonsterName::SlimeAcidSmall,
            ];
            shuffle(&mut pool, rng);
            for &name in &pool {
                push_monster_spawn(effects, name);
            }
        }
        MonsterEncounter::GremlinGang => {
            let mut pool = [
                MonsterName::GremlinWarrior,
                MonsterName::GremlinWarrior,
                MonsterName::GremlinThief,
                MonsterName::GremlinThief,
                MonsterName::GremlinFat,
                MonsterName::GremlinFat,
                MonsterName::GremlinTsundere,
                MonsterName::GremlinWizard,
            ];
            shuffle(&mut pool, rng);
            for &name in &pool[..4] {
                push_monster_spawn(effects, name);
            }
        }
        MonsterEncounter::ExordiumThugs => {
            push_monster_spawn(effects, pick_wildlife_weak(rng));
            push_monster_spawn(effects, pick_humanoid_strong(rng));
        }
        MonsterEncounter::ExordiumWildlife => {
            push_monster_spawn(effects, pick_wildlife_strong(rng));
            push_monster_spawn(effects, pick_wildlife_weak(rng));
        }
        MonsterEncounter::GremlinNob => push_monster_spawn(effects, MonsterName::GremlinNob),
        MonsterEncounter::Lagavulin => push_monster_spawn(effects, MonsterName::Lagavulin),
        MonsterEncounter::ThreeSentries => {
            for _ in 0..3 {
                push_monster_spawn(effects, MonsterName::Sentry);
            }
        }
        MonsterEncounter::TheGuardian => push_monster_spawn(effects, MonsterName::TheGuardian),
        MonsterEncounter::Hexaghost => push_monster_spawn(effects, MonsterName::Hexaghost),
        MonsterEncounter::SlimeBoss => push_monster_spawn(effects, MonsterName::SlimeBoss),
    }
}

#[cfg(test)]
mod tests {
    use crate::action::Action;
    use crate::action::handle_action;
    use crate::action::recompute_legal_actions;
    use crate::consts::MAP_WIDTH;
    use crate::effect::Amount;
    use crate::effect::Effect;
    use crate::effect::EffectKind;
        use crate::effect::Target;
    use crate::engine::process_effect_queue;
    use crate::game::GameState;
    use crate::game::Location;
    use crate::game::create_game_state;
    use crate::types::DeltaSign;
    use crate::types::EventName;
    use crate::types::MonsterName;
    use crate::types::RelicName;
    use crate::types::RoomKind;
    use crate::types::Screen;
    use crate::engine::test_support::grant_relic;

    fn game_with_relic(relic: RelicName) -> GameState {
        let mut state = create_game_state(0, 42, false);
        grant_relic(relic, &mut state.id_relics, &mut state.entities);
        state
    }

    // Repurpose a row-0 room as `kind` and enter it
    fn enter_room(state: &mut GameState, kind: RoomKind) {
        let x = (0..MAP_WIDTH)
            .find(|&x| state.id_rooms[0][x].is_some())
            .expect("row 0 has a room");
        let id_room = state.id_rooms[0][x].unwrap();
        state.entities[id_room].room_kind = kind;
        state.location = Location::Overworld { y: 0, x };
        state.effect_queue.push_back(Effect {
            kind: EffectKind::RoomEnter,
            id_source: None,
            target: Target::Direct(None),
        });
        process_effect_queue(state);
    }

    fn gold(state: &GameState) -> u16 {
        state.entities[state.id_character].character_gold
    }

    #[test]
    fn event_pools_draw_without_replacement_and_gate_the_cleric() {
        let mut state = create_game_state(0, 9, false);
        state.entities[state.id_character].character_gold = 0;
        let mut drawn = 0;
        while super::draw_event(&mut state).is_some() {
            drawn += 1;
        }
        // The Cleric stays pooled below 35 gold
        assert_eq!(drawn, 8);
        assert_eq!(state.pool_events, vec![EventName::TheCleric]);
        state.entities[state.id_character].character_gold = 35;
        assert_eq!(super::draw_event(&mut state), Some(EventName::TheCleric));
        assert_eq!(super::draw_event(&mut state), None);
        for _ in 0..4 {
            assert!(super::draw_shrine(&mut state).is_some());
        }
        assert!(super::draw_shrine(&mut state).is_none());
    }

    #[test]
    fn meal_ticket_heals_on_shop_entry() {
        let mut state = game_with_relic(RelicName::MealTicket);
        let id_character = state.id_character;
        state.entities[id_character].vitals.health -= 20;
        let hp_before = state.entities[id_character].vitals.health;
        enter_room(&mut state, RoomKind::Shop);
        assert_eq!(state.entities[id_character].vitals.health, hp_before + 15);
    }

    #[test]
    fn maw_bank_pays_until_gold_is_spent_at_a_shop() {
        let mut state = game_with_relic(RelicName::MawBank);
        let gold0 = gold(&state);
        enter_room(&mut state, RoomKind::RestSite);
        assert_eq!(gold(&state), gold0 + 12);
        // Gold lost outside a shop does not deactivate it
        state.screen = Screen::Event;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
        process_effect_queue(&mut state);
        enter_room(&mut state, RoomKind::RestSite);
        assert_eq!(gold(&state), gold0 + 19);
        // Spending at a shop kills it for the rest of the run
        state.screen = Screen::Shop;
        state.effect_queue.push_back(Effect {
            kind: EffectKind::GoldDelta {
                sign: DeltaSign::Loss,
                amount: Amount::Absolute(5),
            },
            id_source: None,
            target: Target::Direct(Some(state.id_character)),
        });
        process_effect_queue(&mut state);
        enter_room(&mut state, RoomKind::RestSite);
        assert_eq!(gold(&state), gold0 + 14);
    }

    #[test]
    fn juzu_bracelet_turns_monster_rolls_into_events() {
        let mut state = game_with_relic(RelicName::JuzuBracelet);
        state.unknown_chance_monster = 1.0;
        state.unknown_chance_shop = 0.0;
        state.unknown_chance_treasure = 0.0;
        enter_room(&mut state, RoomKind::Unknown);
        assert_eq!(state.screen, Screen::Event);
    }

    #[test]
    fn tiny_chest_forces_every_fourth_unknown_to_treasure() {
        let mut state = game_with_relic(RelicName::TinyChest);
        let id = state.id_relics[RelicName::TinyChest as usize].unwrap();
        state.entities[id].relic_counter = 3;
        enter_room(&mut state, RoomKind::Unknown);
        assert_eq!(state.screen, Screen::Chest);
        assert_eq!(state.entities[id].relic_counter, 0);
    }

    #[test]
    fn eternal_feather_heals_per_five_deck_cards() {
        let mut state = game_with_relic(RelicName::EternalFeather);
        let id_character = state.id_character;
        state.entities[id_character].vitals.health -= 20;
        let hp_before = state.entities[id_character].vitals.health;
        // Starter deck is 12 cards: (12 / 5) * 3 = 6
        enter_room(&mut state, RoomKind::RestSite);
        assert_eq!(state.entities[id_character].vitals.health, hp_before + 6);
    }

    #[test]
    fn ancient_tea_set_primes_at_rest_and_sips_at_combat_start() {
        let mut state = game_with_relic(RelicName::AncientTeaSet);
        let id = state.id_relics[RelicName::AncientTeaSet as usize].unwrap();
        enter_room(&mut state, RoomKind::RestSite);
        assert_eq!(state.entities[id].relic_counter, 1);
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
        assert_eq!(state.energy.energy_current, 5);
        assert_eq!(state.entities[id].relic_counter, 0);
    }

    #[test]
    fn regal_pillow_heals_more_on_rest() {
        let mut state = game_with_relic(RelicName::RegalPillow);
        let id_character = state.id_character;
        state.entities[id_character].vitals.health = 20;
        enter_room(&mut state, RoomKind::RestSite);
        recompute_legal_actions(&mut state);
        handle_action(&mut state, Action::Rest).unwrap();
        process_effect_queue(&mut state);
        // 30% of 70 max = 21, plus the pillow's 15
        assert_eq!(state.entities[id_character].vitals.health, 56);
    }
}
