// Game loop: step, initialize, FSM determination.

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::character::{spawn_silent, silent_starter_deck};
use crate::effect::Effect;
use crate::map::generate_map;
use crate::engine::process_queue;
use crate::state::*;
use crate::types::*;

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
        fsm: Fsm::Map,
        rng,
        entities: vec![Some(character)],
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
    state.fsm = determine_fsm(state);
}

// ---------------------------------------------------------------------------
// Step
// ---------------------------------------------------------------------------

pub fn step(state: &mut GameState, action: Action) {
    let effects = handle_action(state, action);
    for e in effects {
        state.effect_queue.push_back(e);
    }
    process_queue(state);
    state.fsm = determine_fsm(state);
}

// ---------------------------------------------------------------------------
// FSM determination
// ---------------------------------------------------------------------------

pub fn determine_fsm(state: &GameState) -> Fsm {
    if let Some(front) = state.effect_queue.front() {
        return match front {
            Effect::GameEnd => Fsm::GameOver,
            Effect::AwaitDiscard => Fsm::CombatAwaitDiscard,
            Effect::AwaitMapNode => Fsm::Map,
            Effect::AwaitCardReward => Fsm::CardReward,
            _ => panic!("Unexpected pending effect: {:?}", front),
        };
    }

    if state.card_active.is_some() {
        return Fsm::CombatAwaitTarget;
    }

    match state.map.active_room_type() {
        Some(RoomType::RestSite) => Fsm::RestSite,
        Some(RoomType::CombatMonster) | Some(RoomType::CombatBoss) => Fsm::CombatDefault,
        None => Fsm::Map,
    }
}

