// Game loop: step, initialize, Phase determination.

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::character::{silent_starter_deck, spawn_silent};
use crate::consts::{MAX_COMBAT_CARD_REWARD, MAX_MONSTERS, MAX_SIZE_HAND};
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::process_queue;
use crate::map::generate_map;
use crate::state::*;
use crate::types::Phase;

// Create and initialize
pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    let character = spawn_silent(ascension);
    let deck_templates = silent_starter_deck();

    let mut entities = Vec::with_capacity(256);
    entities.push(character);
    let mut id_deck = Vec::with_capacity(deck_templates.len());
    for card in deck_templates {
        let id_card = entities.len();
        entities.push(card);
        id_deck.push(id_card);
    }

    let (id_rooms, location) = generate_map(&mut rng, &mut entities);

    // Seed the queue with the initial RoomSelect prompt so the player
    // starts halted on the first map pick.
    let mut effect_queue = VecDeque::with_capacity(64);
    effect_queue.push_back(Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
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
        id_character: 0,
        id_monsters: [0; MAX_MONSTERS],
        monster_count: 0,
        energy: Energy { current: 3, max: 3 },
        id_deck,
        id_draw_pile: Vec::with_capacity(64),
        id_hand: Vec::with_capacity(MAX_SIZE_HAND),
        id_discard_pile: Vec::with_capacity(64),
        id_exhaust_pile: Vec::with_capacity(32),
        id_card_target: None,
        id_card_rewards: Vec::with_capacity(MAX_COMBAT_CARD_REWARD),
        id_rooms,
        location,
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
