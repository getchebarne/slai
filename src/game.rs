// Game loop: step, initialize, Phase determination.

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::character::{silent_starter_deck, spawn_silent};
use crate::consts::MAX_MONSTERS;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::process_queue;
use crate::map::{entitize_map, generate_map};
use crate::state::*;
use crate::types::{EntityId, Phase, RoomType};

// Create + initialize
pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    let character = Entity {
        kind: EntityKind::Character(spawn_silent(ascension)),
    };

    let deck_templates = silent_starter_deck();
    let map_grid = generate_map(&mut rng);

    let mut entities = vec![character];
    let mut deck = Vec::with_capacity(deck_templates.len());
    for card in deck_templates {
        let id = EntityId(entities.len() as u32);
        entities.push(Entity {
            kind: EntityKind::Card(card),
        });
        deck.push(id);
    }

    // Entitize the map: push each node into entities and rewrite the grid
    // to reference them by id.
    let map = entitize_map(map_grid, &mut entities);

    // Seed the queue with the initial SelectMapNode prompt so the player
    // starts halted on the first map pick.
    let mut effect_queue = VecDeque::new();
    effect_queue.push_back(Effect {
        kind: EffectKind::SelectMapNode,
        source: None,
        target: Target::Resolve {
            candidates: CandidatePool::MapNodeNextRow,
            selection: SelectionKind::Input { count: 1 },
        },
    });

    let mut state = GameState {
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
        effect_queue,
    };

    // Run the queue so the initial halt registers.
    process_queue(&mut state);
    state.phase = determine_phase(&state);
    state
}

// Step
pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    let was_halted = is_halt_phase(state.phase);

    // Get effects from action handler
    let effects = handle_action(state, action)?;

    if was_halted {
        // Consume the unresolved halt effect at the queue front. Player
        // action handlers never touch the queue; step() owns this cleanup.
        state.effect_queue.pop_front();
    }

    // Put effects into the queue in order
    for effect in effects {
        state.effect_queue.push_back(effect);
    }

    // Process the queue (peek-before-pop)
    process_queue(state);

    // Determine new phase from the current queue front
    state.phase = determine_phase(state);
    Ok(())
}

fn is_halt_phase(phase: Phase) -> bool {
    matches!(
        phase,
        Phase::Map | Phase::CardReward | Phase::CombatAwaitInput | Phase::GameOver
    )
}

// Phase determination: peek the queue front and map halt effects to phases.
// Anything else falls through to room-state derivation. Read-only peek.
pub fn determine_phase(state: &GameState) -> Phase {
    if let Some(effect) = state.effect_queue.front() {
        match effect.kind {
            EffectKind::GameOver => return Phase::GameOver,
            EffectKind::SelectMapNode => return Phase::Map,
            EffectKind::SelectCardReward => return Phase::CardReward,
            _ => {}
        }
        if let Target::Resolve {
            selection: SelectionKind::Input { .. },
            ..
        } = effect.target
        {
            return Phase::CombatAwaitInput;
        }
    }

    match state.map.active_room_type(&state.entities) {
        Some(RoomType::RestSite) => Phase::RestSite,
        Some(RoomType::CombatMonster) | Some(RoomType::CombatBoss) => Phase::CombatDefault,
        None => Phase::Map,
    }
}
