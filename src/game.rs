// Game loop: step, initialize, Phase determination.

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::character::{silent_starter_deck, spawn_silent};
use crate::consts::MAX_MONSTERS;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::process_queue;
use crate::entities::{make_entity_from_card, make_entity_from_character};
use crate::map::{entitize_map, generate_map};
use crate::state::*;
use crate::types::Phase;

// Create and initialize
pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    let character = make_entity_from_character(spawn_silent(ascension));

    let deck_templates = silent_starter_deck();
    let map_grid = generate_map(&mut rng);

    let mut entities = vec![character];
    let mut deck = Vec::with_capacity(deck_templates.len());
    for card in deck_templates {
        let id = entities.len();
        entities.push(make_entity_from_card(card));
        deck.push(id);
    }

    // Entitize the map: push each node into entities and rewrite the grid
    // to reference them by id.
    let map = entitize_map(map_grid, &mut entities);

    // Seed the queue with the initial RoomSelect prompt so the player
    // starts halted on the first map pick.
    let mut effect_queue = VecDeque::new();
    effect_queue.push_back(Effect {
        kind: EffectKind::RoomSelect,
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::NextRowRooms,
            selection: SelectionKind::Input { count: 1 },
        },
    });

    let mut state = GameState {
        ascension,
        phase: Phase::Map,
        rng,
        entities,
        character: 0,
        monsters: [0; MAX_MONSTERS],
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
        effect_queue,
    };

    // Run the queue so the initial halt registers.
    process_queue(&mut state);
    state
}

pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    let effects = handle_action(state, action)?;

    // Push action effects to the FRONT of the queue (in order). When the
    // engine halts mid-chain (e.g., a discard prompt during a card play),
    // the remaining effects from the interrupted chain are still in the
    // queue. The player's response must be inserted before them so the
    // response processes first, then the chain resumes.
    for effect in effects.into_iter().rev() {
        state.effect_queue.push_front(effect);
    }

    process_queue(state);
    Ok(())
}
