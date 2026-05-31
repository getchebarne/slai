use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use strum::EnumCount;

use crate::action::Action;
use crate::action::handle_action;
use crate::action::recompute_legal_actions;
use crate::character::get_silent_starter_deck;
use crate::character::spawn_silent;
use crate::consts::DISCOVER_PICK_COUNT;
use crate::consts::ENCOUNTER_POOL_CAPACITY_ELITE;
use crate::consts::ENCOUNTER_POOL_CAPACITY_NORMAL;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::consts::MAX_CANDIDATES;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::consts::MAX_EFFECTS_PER_HANDLER;
use crate::consts::MAX_ENTITIES;
use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_SIZE_DECK;
use crate::consts::MAX_SIZE_HAND;
use crate::consts::UNKNOWN_CHANCE_BASE_MONSTER;
use crate::consts::UNKNOWN_CHANCE_BASE_SHOP;
use crate::consts::UNKNOWN_CHANCE_BASE_TREASURE;
use crate::effect::Effect;
use crate::engine::process_queue;
use crate::entity::Entity;
use crate::map::generate_map;
use crate::monsters::encounters::generate_act1_monsters;
use crate::monsters::encounters::pick_act1_boss;
use crate::relics::get_relic;
use crate::types::*;
use crate::utils::push_entity;

#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub energy_current: u8,
    pub energy_max: u8,
}

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
    pub buf_effects: Vec<Effect>,

    // Per-resolve candidate buffer; cleared before each use
    pub buf_candidates: Vec<usize>,

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

    // `?`-room drift state; event chance = 1 - sum(others)
    pub unknown_chance_monster: f32,
    pub unknown_chance_shop: f32,
    pub unknown_chance_treasure: f32,

    // Events already surfaced this run (no-repeat filter)
    pub events_seen: Vec<EventName>,

    // Potion drop swing: chance = POTION_DROP_CHANCE_BASE + potion_drop_mod
    pub potion_drop_mod: i8,

    pub screen: Screen,
    pub game_over: bool,

    // Combat working memory; meaningful when active = Combat
    pub id_hand: Vec<usize>,
    pub id_pile_draw: Vec<usize>,
    pub id_pile_discard: Vec<usize>,
    pub id_pile_exhaust: Vec<usize>,
    pub id_monsters: [Option<usize>; MAX_MONSTERS],
    pub id_picked_monster: Option<usize>,
    pub energy: Energy,
    pub this_turn_discards: u8,
    pub this_turn_attacks: u8,
    pub this_combat_damage_instances_taken: u8,
    pub this_combat_escaped: bool,
    pub id_card_last_drawn: Option<usize>,
    pub id_card_nightmare: Option<usize>,
    pub id_discover: Vec<usize>,

    // Reward working memory; meaningful when active = Reward
    pub reward_id_cards: Vec<usize>,
    pub reward_id_relic: Option<usize>,
    pub reward_id_potion: Option<usize>,
    pub reward_gold: Option<u16>,

    // Event working memory; meaningful when active = Event
    pub id_event: Option<usize>,

    // Cached legal-action set; recomputed at every settle point, source of truth for action validity
    pub legal_actions: Vec<Action>,

    // When set, step auto-applies any forced move (exactly one legal action) until a real choice appears
    pub fast_mode: bool,
}

// Create and initialize
pub fn create_game_state(ascension: u8, seed: u64, fast_mode: bool) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    // Initialize empty entities arena
    let mut entities: Vec<Entity> = Vec::with_capacity(MAX_ENTITIES);

    // Initialize character
    let character = spawn_silent(ascension);
    push_entity(&mut entities, character);

    // Innate start relic
    let id_snake_ring = push_entity(&mut entities, get_relic(RelicName::SnakeRing));
    let mut id_relics: [Option<usize>; RelicName::COUNT] = [None; RelicName::COUNT];
    id_relics[RelicName::SnakeRing as usize] = Some(id_snake_ring);

    // Initialize starter deck
    let deck_starter = get_silent_starter_deck();
    let mut id_deck: Vec<usize> = Vec::with_capacity(MAX_SIZE_DECK);
    for card in deck_starter {
        let id_card = push_entity(&mut entities, card);
        id_deck.push(id_card);
    }

    // Initialize map
    let (id_rooms, location) = generate_map(&mut rng, &mut entities);

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

    // Start unhalted on Screen::Map; the empty queue drains and legal_actions_map enumerates row-0 picks
    let effect_queue = VecDeque::with_capacity(64);

    let mut state = GameState {
        ascension,
        rng,
        entities,
        id_character: 0,
        id_deck,
        id_relics,
        id_rooms,
        location,
        encounter_pool_normal,
        encounter_pool_elite,
        encounter_boss,
        effect_queue,
        buf_effects: Vec::with_capacity(MAX_EFFECTS_PER_HANDLER),
        buf_candidates: Vec::with_capacity(MAX_CANDIDATES),
        effect_pending: None,
        unknown_chance_monster: UNKNOWN_CHANCE_BASE_MONSTER,
        unknown_chance_shop: UNKNOWN_CHANCE_BASE_SHOP,
        unknown_chance_treasure: UNKNOWN_CHANCE_BASE_TREASURE,
        events_seen: Vec::with_capacity(EventName::COUNT),
        potion_drop_mod: 0,

        screen: Screen::Map,
        game_over: false,
        id_hand: Vec::with_capacity(MAX_SIZE_HAND),
        id_pile_draw: Vec::with_capacity(MAX_SIZE_DECK),
        id_pile_discard: Vec::with_capacity(MAX_SIZE_DECK),
        id_pile_exhaust: Vec::with_capacity(MAX_SIZE_DECK),
        id_monsters: [None; MAX_MONSTERS],
        id_picked_monster: None,
        energy: Energy {
            energy_current: 3,
            energy_max: 3,
        },
        this_turn_discards: 0,
        this_turn_attacks: 0,
        this_combat_damage_instances_taken: 0,
        this_combat_escaped: false,
        id_card_last_drawn: None,
        id_card_nightmare: None,
        id_discover: Vec::with_capacity(DISCOVER_PICK_COUNT as usize),
        reward_id_cards: Vec::with_capacity(MAX_COMBAT_CARD_REWARD),
        reward_id_relic: None,
        reward_id_potion: None,
        reward_gold: None,
        id_event: None,
        legal_actions: Vec::new(),
        fast_mode,
    };

    // Run the queue so the initial halt registers
    process_queue(&mut state);
    recompute_legal_actions(&mut state);
    state
}

pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    let effects = handle_action(state, action)?;
    enqueue_and_run(state, effects);
    recompute_legal_actions(state);
    if state.fast_mode {
        auto_advance(state);
    }
    Ok(())
}

// Push to FRONT (reversed) so action effects resolve before any halt-interrupted chain, then drain
fn enqueue_and_run(state: &mut GameState, effects: Vec<Effect>) {
    for effect in effects.into_iter().rev() {
        state.effect_queue.push_front(effect);
    }
    process_queue(state);
}

// Skip forced moves: while exactly one legal action exists, apply it
fn auto_advance(state: &mut GameState) {
    let mut guard = 0;
    while !state.game_over && state.legal_actions.len() == 1 {
        guard += 1;
        assert!(guard < 1024, "fast_mode auto-advance exceeded 1024 forced moves");
        let only = state.legal_actions[0].clone();
        let effects = handle_action(state, only).expect("cached single legal action must be valid");
        enqueue_and_run(state, effects);
        recompute_legal_actions(state);
    }
}
