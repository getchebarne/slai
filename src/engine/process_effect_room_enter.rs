use rand::Rng;

use crate::consts::CHEST_SMALL_PCT;
use crate::consts::CHEST_SMALL_PLUS_MEDIUM_PCT;
use crate::consts::EVENT_SPECIAL_CHANCE;
use crate::consts::UNKNOWN_CHANCE_BASE_MONSTER;
use crate::consts::UNKNOWN_CHANCE_BASE_SHOP;
use crate::consts::UNKNOWN_CHANCE_BASE_TREASURE;
use crate::effect::Amount;
use crate::effect::CandidateFilter;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::Target;
use crate::events::BEGGAR_COST_PURGE;
use crate::events::spawn_event;
use crate::game::GameState;
use crate::game::Location;
use crate::map::get_active_room_kind;
use crate::monsters::encounters::spawn_encounter_monsters;
use crate::relics::iter_owned_relics;
use crate::types::ChestKind;
use crate::types::DeltaSign;
use crate::types::EventName;
use crate::types::RelicName;
use crate::types::RoomKind;
use crate::utils::candidate_matches;
use crate::utils::has_relic;

pub fn process_effect_room_enter(state: &mut GameState) {
    // Maw Bank: 12 gold on every room entry until deactivated
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
    let room_kind = get_active_room_kind(&state.id_rooms, state.location, &state.entities).unwrap();
    let room_kind_resolved = if room_kind == RoomKind::Unknown {
        // Ssserpent Head: gain 50 gold on entering a "?" room, whatever it resolves to
        if has_relic(&state.id_relics, RelicName::SsserpentHead) {
            state.effect_queue.push_back(Effect {
                kind: EffectKind::GoldDelta {
                    sign: DeltaSign::Gain,
                    amount: Amount::Absolute(50),
                },
                id_source: None,
                target: Target::Direct(None),
            });
        }
        roll_unknown_room(state)
    } else {
        room_kind
    };

    match room_kind_resolved {
        RoomKind::CombatBoss => {
            // Spawn boss
            let encounter = state.encounter_boss;
            spawn_encounter_monsters(state, encounter);

            // Pantograph: boss fights open with a 25 HP heal
            if has_relic(&state.id_relics, RelicName::Pantograph) {
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::HealthDelta {
                        sign: DeltaSign::Gain,
                        amount: Amount::Absolute(25),
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }
        }
        RoomKind::CombatMonster => {
            // Pop an encounter and spawn its monsters
            let encounter = state.encounter_pool_normal.remove(0);
            spawn_encounter_monsters(state, encounter);
        }
        RoomKind::CombatElite => {
            // Pop an encounter and spawn its monsters
            let encounter = state.encounter_pool_elite.remove(0);
            spawn_encounter_monsters(state, encounter);
        }
        RoomKind::RestSite => {
            state.rest_site.consumed = false;
            state.rest_site.active = true;

            // Eternal Feather: 3 HP per 5 deck Cards on arrival
            if has_relic(&state.id_relics, RelicName::EternalFeather) {
                let heal = (state.id_card_deck.len() / 5) * 3;
                state.effect_queue.push_back(Effect {
                    kind: EffectKind::HealthDelta {
                        sign: DeltaSign::Gain,
                        amount: Amount::Absolute(heal as u16),
                    },
                    id_source: None,
                    target: Target::Direct(Some(state.id_character)),
                });
            }

            // Ancient Tea Set: prime for the next combat
            if let Some(id) = state.id_relics[RelicName::AncientTeaSet as usize] {
                state.entities[id].relic_counter = 1;
            }
        }
        RoomKind::Treasure => {
            // Roll the chest kind into the context
            let roll = state.rng.random_range(0..100) as u8;
            state.chest.chest_kind = if roll < CHEST_SMALL_PCT {
                ChestKind::Small
            } else if roll < CHEST_SMALL_PLUS_MEDIUM_PCT {
                ChestKind::Medium
            } else {
                ChestKind::Large
            };
            state.chest.chest_opened = false;
            state.chest.active = true;
        }
        RoomKind::EventRoom => {
            let name = draw_random_event(state).expect("Event room with no drawable event");
            let (kind, id_event_options) = spawn_event(state, name);
            state.event.event_kind = kind;
            state.event.consumed = false;
            state.event.id_event_options = id_event_options;
            state.event.active = true;
        }
        // ShopBuild fills the shop context; until it runs the focus stays Map
        RoomKind::Shop => {
            // Meal Ticket: Heal 15 on shop enter
            if has_relic(&state.id_relics, RelicName::MealTicket) {
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
}

// Resolve a "?" room into a concrete kind, then drift the running tallies
fn roll_unknown_room(state: &mut GameState) -> RoomKind {
    // Tiny Chest: every 4th ? room is forced Treasure; drift still runs as if rolled
    let force_treasure = if let Some(id) = state.id_relics[RelicName::TinyChest as usize] {
        // Increase counter
        let counter = &mut state.entities[id].relic_counter;
        *counter += 1;

        // Reset
        if *counter >= 4 {
            *counter = 0;
            true
        } else {
            false
        }
    } else {
        false
    };

    // Resolve the room kind
    let mut room_kind_resolved = if force_treasure {
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
    state.unknown_chance_monster = if room_kind_resolved == RoomKind::CombatMonster {
        UNKNOWN_CHANCE_BASE_MONSTER
    } else {
        state.unknown_chance_monster + UNKNOWN_CHANCE_BASE_MONSTER
    };
    state.unknown_chance_shop = if room_kind_resolved == RoomKind::Shop {
        UNKNOWN_CHANCE_BASE_SHOP
    } else {
        state.unknown_chance_shop + UNKNOWN_CHANCE_BASE_SHOP
    };
    state.unknown_chance_treasure = if room_kind_resolved == RoomKind::Treasure {
        UNKNOWN_CHANCE_BASE_TREASURE
    } else {
        state.unknown_chance_treasure + UNKNOWN_CHANCE_BASE_TREASURE
    };

    // Juzu Bracelet: a monster resolution becomes an event (after the drift settles)
    if room_kind_resolved == RoomKind::CombatMonster
        && has_relic(&state.id_relics, RelicName::JuzuBracelet)
    {
        room_kind_resolved = RoomKind::EventRoom;
    }

    room_kind_resolved
}

// 25% special pool (shrines + one-time events), else event pool; an exhausted
// pool falls back to the other, both empty -> None
fn draw_random_event(state: &mut GameState) -> Option<EventName> {
    if state.rng.random_range(0.0..1.0f32) < EVENT_SPECIAL_CHANCE {
        draw_event_special(state).or_else(|| draw_event(state))
    } else {
        draw_event(state).or_else(|| draw_event_special(state))
    }
}

fn draw_event(state: &mut GameState) -> Option<EventName> {
    // Draw-gated events stay pooled until eligible (source: getEvent's filters)
    let gold = state.entities[state.id_character].character_gold;
    let floor = match state.location {
        Location::Overworld { y, .. } => y + 1,
        _ => 0,
    };
    let eligible: Vec<usize> = state
        .pool_events
        .iter()
        .enumerate()
        .filter(|&(_, &name)| match name {
            EventName::TheCleric => gold >= 35,
            EventName::Beggar => gold >= BEGGAR_COST_PURGE,
            EventName::Colosseum => floor > 8,
            EventName::Mushrooms | EventName::DeadAdventurer => floor > 6,
            _ => true,
        })
        .map(|(i, _)| i)
        .collect();

    // Early return if empty
    if eligible.is_empty() {
        return None;
    }

    // Roll, pop from `pool_events` and return the rolled event's name
    let idx = eligible[state.rng.random_range(0..eligible.len())];
    Some(state.pool_events.swap_remove(idx))
}

fn draw_event_special(state: &mut GameState) -> Option<EventName> {
    // Draw-gated specials stay pooled until eligible
    let gold = state.entities[state.id_character].character_gold;

    // Calculate if there's any removable curses in the deck. This gates "The Divine Fountain"
    let has_removable_curse = state.id_card_deck.iter().any(|&id| {
        candidate_matches(
            CandidateFilter::PurgeableCurse,
            id,
            &state.entities[id],
            None,
            None,
        )
    });

    // Calculate eligible specials
    let eligible: Vec<usize> = state
        .pool_event_special
        .iter()
        .enumerate()
        .filter(|&(_, &name)| match name {
            EventName::TheDivineFountain => has_removable_curse,
            EventName::TheWomanInBlue => gold >= 50,
            EventName::TheJoust => gold >= 50,
            EventName::Designer => gold >= 75,
            EventName::Nloth => iter_owned_relics(&state.id_relics).nth(1).is_some(),
            EventName::KnowingSkull => state.entities[state.id_character].vitals.health > 12,
            _ => true,
        })
        .map(|(i, _)| i)
        .collect();

    // Early return if empty
    if eligible.is_empty() {
        return None;
    }

    // Roll, pop from `pool_event_special` and return the rolled special's name
    let idx = eligible[state.rng.random_range(0..eligible.len())];
    Some(state.pool_event_special.swap_remove(idx))
}
