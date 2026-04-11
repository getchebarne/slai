// Game loop: step, initialize, Phase determination.

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::character::{silent_starter_deck, spawn_silent};
use crate::consts::MAX_MONSTERS;
use crate::engine::{HaltReason, process_queue};
use crate::map::generate_map;
use crate::state::*;
use crate::types::{EntityId, Phase, RoomType};

// Create + initialize
pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    let character = Entity {
        kind: EntityKind::Character(spawn_silent(ascension)),
    };

    let deck_templates = silent_starter_deck();
    let map = generate_map(&mut rng);

    let mut entities = vec![character];
    let mut deck = Vec::with_capacity(deck_templates.len());
    for card in deck_templates {
        let id = EntityId(entities.len() as u32);
        entities.push(Entity {
            kind: EntityKind::Card(card),
        });
        deck.push(id);
    }

    GameState {
        ascension,
        phase: Phase::Map,
        rng,
        entities,
        character: EntityId(0),
        monsters: [EntityId(0); MAX_MONSTERS],
        monster_count: 0,
        energy: Energy { current: 3, max: 3 },
        deck,
        draw_pile: Vec::new(),
        hand: Vec::new(),
        discard_pile: Vec::new(),
        exhaust_pile: Vec::new(),
        card_target: None,
        card_rewards: Vec::new(),
        map,
        effect_queue: VecDeque::new(),
    }
}

pub fn initialize(state: &mut GameState) {
    // Fresh game starts waiting for the player's first map node pick.
    state.phase = determine_phase(Some(HaltReason::AwaitMapNode), state);
}

// Step
pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    let effects = handle_action(state, action)?;
    for effect in effects {
        state.effect_queue.push_back(effect);
    }
    let halt = process_queue(state);
    state.phase = determine_phase(halt, state);
    Ok(())
}

// Phase determination. `halt` is the ephemeral result of the most recent
// `process_queue` call. When it's `None`, the engine is mid-room and phase is
// derived from the active room.
pub fn determine_phase(halt: Option<HaltReason>, state: &GameState) -> Phase {
    match halt {
        Some(HaltReason::GameOver) => return Phase::GameOver,
        Some(HaltReason::AwaitDiscard) => return Phase::CombatAwaitDiscard,
        Some(HaltReason::AwaitMapNode) => return Phase::Map,
        Some(HaltReason::AwaitCardReward) => return Phase::CardReward,
        None => {}
    }

    match state.map.active_room_type() {
        Some(RoomType::RestSite) => Phase::RestSite,
        Some(RoomType::CombatMonster) | Some(RoomType::CombatBoss) => Phase::CombatDefault,
        None => Phase::Map,
    }
}
