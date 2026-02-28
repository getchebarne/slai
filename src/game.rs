// Game loop: step, initialize, Phase determination.

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::consts::MAX_MONSTERS;
use crate::character::{silent_starter_deck, spawn_silent};
use crate::effect::Effect;
use crate::engine::process_queue;
use crate::map::generate_map;
use crate::state::*;
use crate::types::{EntityId, Phase, RoomType};

// ---------------------------------------------------------------------------
// Create + initialize
// ---------------------------------------------------------------------------

pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    let character = Entity {
        kind: EntityKind::Character(spawn_silent(ascension)),
    };

    let deck = silent_starter_deck();
    let map = generate_map(&mut rng);

    GameState {
        ascension,
        phase: Phase::Map,
        rng,
        entities: vec![character],
        character: EntityId(0),
        monsters: [EntityId(0); MAX_MONSTERS],
        monster_count: 0,
        energy: Energy { current: 3, max: 3 },
        deck,
        draw_pile: Vec::new(),
        hand: Vec::new(),
        discard_pile: Vec::new(),
        exhaust_pile: Vec::new(),
        card_active: None,
        card_target: None,
        card_rewards: Vec::new(),
        map,
        effect_queue: VecDeque::new(),
    }
}

pub fn initialize(state: &mut GameState) {
    state.effect_queue.push_back(Effect::AwaitMapNode);
    state.phase = determine_phase(state);
}

// ---------------------------------------------------------------------------
// Step
// ---------------------------------------------------------------------------

pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    let effects = handle_action(state, action)?;
    for e in effects {
        state.effect_queue.push_back(e);
    }
    process_queue(state);
    state.phase = determine_phase(state);
    Ok(())
}

// ---------------------------------------------------------------------------
// Phase determination
// ---------------------------------------------------------------------------

pub fn determine_phase(state: &GameState) -> Phase {
    if let Some(front) = state.effect_queue.front() {
        return match front {
            Effect::GameEnd => Phase::GameOver,
            Effect::AwaitDiscard => Phase::CombatAwaitDiscard,
            Effect::AwaitMapNode => Phase::Map,
            Effect::AwaitCardReward => Phase::CardReward,
            _ => panic!("Unexpected pending effect: {:?}", front),
        };
    }

    if state.card_active.is_some() {
        return Phase::CombatAwaitTarget;
    }

    match state.map.active_room_type() {
        Some(RoomType::RestSite) => Phase::RestSite,
        Some(RoomType::CombatMonster) | Some(RoomType::CombatBoss) => Phase::CombatDefault,
        None => Phase::Map,
    }
}
