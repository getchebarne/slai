// Game loop: step, initialize, Phase determination.

use std::collections::VecDeque;
use std::os::unix::process;

use rand::SeedableRng;
use rand::rngs::SmallRng;

use crate::action::{Action, handle_action};
use crate::character::{silent_starter_deck, spawn_silent};
use crate::consts::MAX_MONSTERS;
use crate::effect::{CandidatePool, Effect, EffectKind, SelectionKind, Target};
use crate::engine::{ProcessEffectResult, process_queue};
use crate::map::{entitize_map, generate_map};
use crate::state::*;
use crate::types::{EntityId, Phase, RoomType};

// Create and initialize
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

    // Seed the queue with the initial MapNodeSelect prompt so the player
    // starts halted on the first map pick.
    let mut effect_queue = VecDeque::new();
    effect_queue.push_back(Effect {
        kind: EffectKind::MapNodeSelect,
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
    state
}

// Step
pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    // Get effects from action handler
    let effects = handle_action(state, action)?;

    // Put effects into the queue in order
    for effect in effects {
        state.effect_queue.push_back(effect);
    }

    // Process the queue (peek-before-pop)
    process_queue(state);

    // Determine new phase from the current queue front
    Ok(())
}

// always consume the effect
// if target cannot be resolved: return halt reason (e.g., must select map // card to discard)
// what determines phase:
//   - action to perform (e.g., discard / upgrade )
//   - number of targets
//   - entities to choose from (CandidatePool)
// skippable?
// halt -> PlayerInputNeeded

// Phase determination: peek the queue front and map halt effects to phases.
// Anything else falls through to room-state derivation. Read-only peek.
//     if let Some(effect) = state.effect_queue.front() {
//         match effect.kind {
//             EffectKind::MapNodeSelect => return Phase::Map,
//             _ => {}
//         }
//         if let Target::Resolve {
//             selection: SelectionKind::Input { .. },
//             ..
//         } = effect.target
//         {
//             return Phase::CombatAwaitInput;
//         }
//     }

//     match state.map.active_room_type(&state.entities) {
//         Some(RoomType::RestSite) => Phase::RestSite,
//         Some(RoomType::CombatMonster) | Some(RoomType::CombatBoss) => Phase::CombatDefault,
//         None => Phase::Map,
//     }
// }
