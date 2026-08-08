use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use strum::EnumCount;

use crate::action::Action;
use crate::action::handle_action;
use crate::action::recompute_legal_actions;
use crate::character::get_silent_starter_deck;
use crate::character::spawn_silent;
use crate::consts::ENCOUNTER_POOL_CAPACITY_ELITE;
use crate::consts::ENCOUNTER_POOL_CAPACITY_NORMAL;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::consts::MAX_CANDIDATES;
use crate::consts::MAX_EFFECTS_PER_HANDLER;
use crate::consts::MAX_ENTITIES;
use crate::consts::MAX_SIZE_DECK;
use crate::consts::POTION_SLOTS_DEFAULT;
use crate::consts::POTION_SLOTS_DEFAULT_A11;
use crate::consts::POTION_SLOTS_MAX;
use crate::consts::SHOP_PURGE_COST_BASE;
use crate::consts::UNKNOWN_CHANCE_BASE_MONSTER;
use crate::consts::UNKNOWN_CHANCE_BASE_SHOP;
use crate::consts::UNKNOWN_CHANCE_BASE_TREASURE;
use crate::effect::Effect;
use crate::engine::process_effect_queue;
use crate::entity::Entity;
use crate::events::POOL_EVENT_ACT1;
use crate::events::POOL_EVENT_ACT1_SPECIAL;
use crate::events::spawn_event;
use crate::map::generate_map;
use crate::monsters::encounters::generate_act1_monsters;
use crate::monsters::encounters::pick_act1_boss;
use crate::relics::get_relic;
use crate::types::*;
use crate::utils::push_entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Start,
    Overworld { y: usize, x: usize },
    BossRoom,
}

// GameState: the single source of truth
#[derive(Debug, Clone)]
pub struct GameState {
    // Run config and RNG
    pub ascension: u8,
    pub rng: SmallRng,

    // Engine state
    pub effect_queue: VecDeque<Effect>,

    // Per-handler effect builder; drained back-to-front into queue front
    pub effect_buf: Vec<Effect>,

    // Per-resolve candidate buffer; cleared before each use
    pub effect_candidate_buf: Vec<usize>,

    // Halt overlay; cleared by the action handler that supplies the pick
    pub effect_pending: Option<Effect>,
    pub location: Location,

    // Entities and indices
    pub entities: Vec<Entity>,
    pub id_rooms: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    pub id_character: usize,

    // Monster encounters
    pub encounter_pool_normal: Vec<MonsterEncounter>,
    pub encounter_pool_elite: Vec<MonsterEncounter>,
    pub encounter_boss: MonsterEncounter,

    // Master deck (persists across combats)
    pub id_deck: Vec<usize>,

    // Name-indexed: `id_relics[name as usize]` is `Some(entity_id)` iff owned
    pub id_relics: [Option<usize>; RelicName::COUNT],

    // Slot-indexed belt; `id_potions[slot]` is `Some(entity_id)` iff occupied (duplicates allowed)
    pub id_potions: [Option<usize>; POTION_SLOTS_MAX],
    pub potion_slots_max: u8,

    // `?`-room drift state; event chance = 1 - sum(others)
    pub unknown_chance_monster: f32,
    pub unknown_chance_shop: f32,
    pub unknown_chance_treasure: f32,

    // Run-scoped event draw pools; drawn without replacement, never refilled
    pub pool_events: Vec<EventName>,
    pub pool_event_special: Vec<EventName>,

    // Potion drop swing: chance = POTION_DROP_CHANCE_BASE + potion_drop_mod
    pub potion_drop_mod: i8,

    // Stacked contexts: [0] is always Map (the resting frame), last() is current
    pub mode_stack: Vec<Mode>,
    pub game_over: bool,

    // Removal cost for the whole run: 75 + 25 per purge, never reset
    pub shop_purge_cost_run: u16,

    // Cached legal-action set; recomputed at every settle point, source of truth for action validity
    pub legal_actions: Vec<Action>,

    // When set, step auto-applies any forced move (exactly one legal action) until a real choice appears
    pub fast_mode: bool,

    // When set, the run opens on Neow's blessing before the first map pick
    pub neow: bool,
}

// Create and initialize
pub fn create_game_state(ascension: u8, seed: u64, fast_mode: bool, neow: bool) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    // Initialize empty entities arena
    let mut entities: Vec<Entity> = Vec::with_capacity(MAX_ENTITIES);

    // Initialize character
    let character = spawn_silent(ascension);
    push_entity(&mut entities, character);

    // Innate start Relic
    let id_snake_ring = push_entity(&mut entities, get_relic(RelicName::SnakeRing));
    let mut id_relics: [Option<usize>; RelicName::COUNT] = [None; RelicName::COUNT];
    id_relics[RelicName::SnakeRing as usize] = Some(id_snake_ring);

    // Belt capacity is a run-level rule (3, or 2 at ascension 11+); slots start empty
    let id_potions: [Option<usize>; POTION_SLOTS_MAX] = [None; POTION_SLOTS_MAX];
    let potion_slots_max = if ascension >= 11 {
        POTION_SLOTS_DEFAULT_A11
    } else {
        POTION_SLOTS_DEFAULT
    };

    // Initialize starter deck
    let deck_starter = get_silent_starter_deck(ascension);
    let mut id_deck: Vec<usize> = Vec::with_capacity(MAX_SIZE_DECK);
    for card in deck_starter {
        let id_card = push_entity(&mut entities, card);
        id_deck.push(id_card);
    }

    // Initialize map
    let (id_rooms, location) = generate_map(&mut rng, &mut entities, ascension);

    // Pre-generate monster encounters
    let mut encounter_pool_normal: Vec<MonsterEncounter> =
        Vec::with_capacity(ENCOUNTER_POOL_CAPACITY_NORMAL);
    let mut encounter_pool_elite: Vec<MonsterEncounter> =
        Vec::with_capacity(ENCOUNTER_POOL_CAPACITY_ELITE);
    generate_act1_monsters(
        &mut encounter_pool_normal,
        &mut encounter_pool_elite,
        &mut rng,
    );
    let encounter_boss = pick_act1_boss(&mut rng);

    // Start unhalted on Mode::Map; the empty queue drains and legal_actions_map enumerates row-0 picks
    let effect_queue = VecDeque::with_capacity(64);

    let mut state = GameState {
        ascension,
        rng,
        entities,
        id_character: 0,
        id_deck,
        id_relics,
        id_potions,
        potion_slots_max,
        id_rooms,
        location,
        encounter_pool_normal,
        encounter_pool_elite,
        encounter_boss,
        effect_queue,
        effect_buf: Vec::with_capacity(MAX_EFFECTS_PER_HANDLER),
        effect_candidate_buf: Vec::with_capacity(MAX_CANDIDATES),
        effect_pending: None,
        unknown_chance_monster: UNKNOWN_CHANCE_BASE_MONSTER,
        unknown_chance_shop: UNKNOWN_CHANCE_BASE_SHOP,
        unknown_chance_treasure: UNKNOWN_CHANCE_BASE_TREASURE,
        pool_events: POOL_EVENT_ACT1.to_vec(),
        pool_event_special: POOL_EVENT_ACT1_SPECIAL.to_vec(),
        potion_drop_mod: 0,
        mode_stack: vec![Mode::Map],
        game_over: false,
        shop_purge_cost_run: SHOP_PURGE_COST_BASE,
        legal_actions: Vec::new(),
        fast_mode,
        neow,
    };

    // Neow's blessing rests over the Map frame at Location::Start
    if neow {
        let (kind, id_options) = spawn_event(&mut state, EventName::Neow);
        state.mode_stack.push(Mode::Event {
            kind,
            consumed: false,
            id_options,
        });
    }

    // Settle on the resting frame — Neow's options, or the initial row-0 room picks
    recompute_legal_actions(&mut state);
    state
}

pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    // Handle the action. May enqueue elements to `state.effect_queue`
    handle_action(state, action)?;

    // Process `state.effect_queue`
    process_effect_queue(state);

    // Recompute legal actions
    recompute_legal_actions(state);

    // If fast mode is enabled, auto advance  the game until there's more than one legal action
    if state.fast_mode {
        auto_advance(state);
    }
    Ok(())
}

// Skip forced moves: while exactly one lexgal action exists, apply it
fn auto_advance(state: &mut GameState) {
    let mut guard = 0;
    while !state.game_over && state.legal_actions.len() == 1 {
        guard += 1;
        assert!(
            guard < 99,
            "fast_mode auto-advance exceeded 99 forced moves"
        );
        // Get single legal action
        let action = state.legal_actions[0].clone();

        // Keep processing the queue until there's more than one legal action to take
        handle_action(state, action).expect("Cached single legal action must be valid");
        process_effect_queue(state);
        recompute_legal_actions(state);
    }
}
