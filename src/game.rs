// Game loop: step, initialize, FSM determination.

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::card;
use crate::effect::Effect;
use crate::map::generate_map;
use crate::modifier::modifiers_new;
use crate::process::process_queue;
use crate::state::*;
use crate::types::*;

// ---------------------------------------------------------------------------
// Create + initialize
// ---------------------------------------------------------------------------

pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    // Character (Silent)
    let (health, health_max) = silent_health(ascension);
    let character = Character {
        vitals: Vitals {
            health,
            health_max,
            block: 0,
            modifiers: modifiers_new(),
        },
        reward_roll_offset: 5,
    };

    // Starter deck
    let deck = silent_starter_deck();

    // Map
    let map = generate_map(&mut rng);

    GameState {
        ascension,
        fsm: Fsm::Map, // will be set properly by initialize
        rng,
        character,
        energy: Energy { current: 3, max: 3 },
        deck,
        combat_cards: Vec::new(),
        draw_pile: Vec::new(),
        hand: Vec::new(),
        discard_pile: Vec::new(),
        exhaust_pile: Vec::new(),
        card_active: None,
        card_target: None,
        monsters: Vec::new(),
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

// ---------------------------------------------------------------------------
// Silent character
// ---------------------------------------------------------------------------

fn silent_health(ascension: u8) -> (u16, u16) {
    let mut health_max: u16 = 70;
    let mut health: u16 = health_max;

    if ascension >= 14 {
        health_max -= 4;
        health = health_max;
    }
    if ascension >= 6 {
        health = (0.90 * health as f32) as u16;
    }

    (health, health_max)
}

fn silent_starter_deck() -> Vec<Card> {
    vec![
        card::get_card(CardName::Strike, false),
        card::get_card(CardName::Strike, false),
        card::get_card(CardName::Strike, false),
        card::get_card(CardName::Strike, false),
        card::get_card(CardName::Strike, false),
        card::get_card(CardName::Defend, false),
        card::get_card(CardName::Defend, false),
        card::get_card(CardName::Defend, false),
        card::get_card(CardName::Defend, false),
        card::get_card(CardName::Defend, false),
        card::get_card(CardName::Survivor, false),
        card::get_card(CardName::Neutralize, false),
    ]
}
