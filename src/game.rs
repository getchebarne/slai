use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use strum::EnumCount;

use crate::action::Action;
use crate::action::handle_action;
use crate::character::get_silent_starter_deck;
use crate::character::spawn_silent;
use crate::consts::DISCOVER_PICK_COUNT;
use crate::consts::ENCOUNTER_LIST_CAPACITY_ELITE;
use crate::consts::ENCOUNTER_LIST_CAPACITY_NORMAL;
use crate::consts::EVENT_CHANCE_BASE_MONSTER;
use crate::consts::EVENT_CHANCE_BASE_SHOP;
use crate::consts::EVENT_CHANCE_BASE_TREASURE;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::consts::MAX_CANDIDATES;
use crate::consts::MAX_COMBAT_CARD_REWARD;
use crate::consts::MAX_EFFECTS_PER_HANDLER;
use crate::consts::MAX_ENTITIES;
use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_SIZE_DECK;
use crate::consts::MAX_SIZE_HAND;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::process_queue;
use crate::utils::push_entity;
use crate::entity::Entity;
use crate::map::generate_map;
use crate::monsters::encounters::generate_act1_monsters;
use crate::monsters::encounters::pick_act1_boss;
use crate::relics::get_relic;
use crate::types::*;

#[derive(Debug, Clone, Copy)]
pub struct Energy {
    pub current: u8,
    pub max: u8,
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

    // Per-handler effect builder; cleared before each use. Built in execution
    // order then drained back-to-front into effect_queue's front
    pub buf_effects: Vec<Effect>,

    // Per-resolve candidate buffer; cleared before each use
    pub buf_candidates: Vec<usize>,

    // Halt overlay; cleared by the action handler that supplies the pick
    pub pending_effect: Option<Effect>,
    pub location: Location,

    // Entities and indices
    pub entities: Vec<Entity>,
    pub id_rooms: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    pub id_character: usize,

    // Monster encounters
    pub encounter_list_normal: Vec<MonsterEncounter>,
    pub encounter_list_elite: Vec<MonsterEncounter>,
    pub encounter_boss: MonsterEncounter,

    // Master deck (persists across combats)
    pub id_deck: Vec<usize>,

    // Name-indexed: `id_relics[name as usize]` is `Some(entity_id)` iff owned
    pub id_relics: [Option<usize>; RelicName::COUNT],

    // `?`-room drift state; event chance = 1 - sum(others)
    pub event_chance_monster: f32,
    pub event_chance_shop: f32,
    pub event_chance_treasure: f32,

    // Events already surfaced this run (no-repeat filter)
    pub events_seen_this_run: Vec<EventName>,

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
    pub id_monster_picked: Option<usize>,
    pub energy: Energy,
    pub this_turn_discards: u8,
    pub this_turn_attacks_played: u8,
    pub this_combat_damage_instances_taken: u8,
    pub escaped_this_combat: bool,
    pub card_last_drawn: Option<usize>,
    pub id_card_nightmare: Option<usize>,
    pub id_pick: Vec<usize>,

    // Reward working memory; meaningful when active = Reward
    pub reward_id_cards: Vec<usize>,
    pub reward_id_relic: Option<usize>,
    pub reward_id_potion: Option<usize>,
    pub reward_gold: Option<u16>,

    // Event working memory; meaningful when active = Event
    pub id_event: Option<usize>,
}

// Create and initialize
pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
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
    let mut encounter_list_normal: Vec<MonsterEncounter> =
        Vec::with_capacity(ENCOUNTER_LIST_CAPACITY_NORMAL);
    let mut encounter_list_elite: Vec<MonsterEncounter> =
        Vec::with_capacity(ENCOUNTER_LIST_CAPACITY_ELITE);
    generate_act1_monsters(
        &mut encounter_list_normal,
        &mut encounter_list_elite,
        &mut rng,
    );
    let encounter_boss = pick_act1_boss(&mut rng);

    // Seed the queue with the initial RoomSelect prompt so the player
    // starts halted on the first map pick
    let mut effect_queue = VecDeque::with_capacity(64);
    effect_queue.push_back(Effect {
        kind: EffectKind::RoomSelect,
        id_source: None,
        target: Target::Resolve {
            candidate_pool: CandidatePool::NextRowRooms,
            selection_kind: SelectionKind::Input { count: 1 },
        },
    });

    let mut state = GameState {
        ascension,
        rng,
        entities,
        id_character: 0,
        id_deck,
        id_relics,
        id_rooms,
        location,
        encounter_list_normal,
        encounter_list_elite,
        encounter_boss,
        effect_queue,
        buf_effects: Vec::with_capacity(MAX_EFFECTS_PER_HANDLER),
        buf_candidates: Vec::with_capacity(MAX_CANDIDATES),
        pending_effect: None,
        event_chance_monster: EVENT_CHANCE_BASE_MONSTER,
        event_chance_shop: EVENT_CHANCE_BASE_SHOP,
        event_chance_treasure: EVENT_CHANCE_BASE_TREASURE,
        events_seen_this_run: Vec::with_capacity(EventName::COUNT),
        potion_drop_mod: 0,

        screen: Screen::Map,
        game_over: false,
        id_hand: Vec::with_capacity(MAX_SIZE_HAND),
        id_pile_draw: Vec::with_capacity(MAX_SIZE_DECK),
        id_pile_discard: Vec::with_capacity(MAX_SIZE_DECK),
        id_pile_exhaust: Vec::with_capacity(MAX_SIZE_DECK),
        id_monsters: [None; MAX_MONSTERS],
        id_monster_picked: None,
        energy: Energy { current: 3, max: 3 },
        this_turn_discards: 0,
        this_turn_attacks_played: 0,
        this_combat_damage_instances_taken: 0,
        escaped_this_combat: false,
        card_last_drawn: None,
        id_card_nightmare: None,
        id_pick: Vec::with_capacity(DISCOVER_PICK_COUNT as usize),
        reward_id_cards: Vec::with_capacity(MAX_COMBAT_CARD_REWARD),
        reward_id_relic: None,
        reward_id_potion: None,
        reward_gold: None,
        id_event: None,
    };

    // Run the queue so the initial halt registers
    process_queue(&mut state);
    state
}

pub fn step(state: &mut GameState, action: Action) -> Result<(), String> {
    let effects = handle_action(state, action)?;

    // Push action effects to the FRONT of the queue (in order). When the
    // engine halts mid-chain (e.g., a discard prompt during a card play),
    // the remaining effects from the interrupted chain are still in the
    // queue. The player's response must be inserted before them so the
    // response processes first, then the chain resumes
    for effect in effects.into_iter().rev() {
        state.effect_queue.push_front(effect);
    }

    process_queue(state);
    Ok(())
}
