// GameState definition, game loop: step, initialize, Phase determination

use std::collections::VecDeque;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use strum::EnumCount;

use crate::action::Action;
use crate::action::handle_action;
use crate::character::get_silent_starter_deck;
use crate::character::spawn_silent;
use crate::consts::ENCOUNTER_LIST_ELITE_CAPACITY;
use crate::consts::ENCOUNTER_LIST_NORMAL_CAPACITY;
use crate::consts::EVENT_CHANCE_MONSTER_BASE;
use crate::consts::EVENT_CHANCE_SHOP_BASE;
use crate::consts::EVENT_CHANCE_TREASURE_BASE;
use crate::consts::MAP_HEIGHT;
use crate::consts::MAP_WIDTH;
use crate::consts::MAX_MONSTERS;
use crate::consts::MAX_SIZE_HAND;
use crate::effect::CandidatePool;
use crate::effect::Effect;
use crate::effect::EffectKind;
use crate::effect::SelectionKind;
use crate::effect::Target;
use crate::engine::process_queue;
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
    pub phase: Phase,
    pub effect_queue: VecDeque<Effect>,
    pub location: Location,
    pub energy: Energy,

    // Entities and indices
    pub entities: Vec<Entity>,
    pub id_rooms: [[Option<usize>; MAP_WIDTH]; MAP_HEIGHT],
    pub id_character: usize,

    // Combat state
    pub id_monsters: [usize; MAX_MONSTERS],
    pub monster_count: u8,
    pub id_card_target: Option<usize>,

    // Monster encounters
    pub encounter_list_normal: Vec<MonsterEncounter>,
    pub encounter_list_elite: Vec<MonsterEncounter>,
    pub encounter_boss: MonsterEncounter,

    // Card piles
    pub id_deck: Vec<usize>,
    pub id_pile_draw: Vec<usize>,
    pub id_hand: Vec<usize>,
    pub id_pile_discard: Vec<usize>,
    pub id_pile_exhaust: Vec<usize>,

    // Name-indexed: `id_relics[name as usize]` is `Some(entity_id)` iff owned
    pub id_relics: [Option<usize>; RelicName::COUNT],

    // Needed for Escape Plan
    pub card_last_drawn: Option<usize>,

    // Per-turn counters; reset in process_effect_turn_end_character
    pub this_turn_discards: u8,
    pub this_turn_attacks_played: u8,

    // Per-combat counter; reset at combat_start
    pub this_combat_damage_instances_taken: u8,
    // Suppresses normal-room gold reward when any monster escaped
    pub escaped_this_combat: bool,

    // `?`-room drift state; event chance = 1 - sum(others)
    pub event_chance_monster: f32,
    pub event_chance_shop: f32,
    pub event_chance_treasure: f32,

    // Potion drop swing: chance = POTION_DROP_CHANCE_BASE + potion_drop_mod
    pub potion_drop_mod: i8,

    // Discovery picks: entity ids of the 3 cards offered; halts on CombatAwaitDiscover
    pub id_card_discover: Vec<usize>,

    // Nightmare-pending template snapshot id; flushed at next TurnStart
    pub id_card_nightmare: Option<usize>,
}

// Create and initialize
pub fn create_game_state(ascension: u8, seed: u64) -> GameState {
    let mut rng = SmallRng::seed_from_u64(seed);

    // Initialize empty entities vector
    let mut entities = Vec::with_capacity(256);

    // Initialize character
    let character = spawn_silent(ascension);
    entities.push(character);

    // Innate start relic
    let id_snake_ring = entities.len();
    entities.push(get_relic(RelicName::SnakeRing));
    let mut id_relics: [Option<usize>; RelicName::COUNT] = [None; RelicName::COUNT];
    id_relics[RelicName::SnakeRing as usize] = Some(id_snake_ring);

    // Initialize starter deck
    let deck_starter = get_silent_starter_deck();
    let mut id_deck = Vec::with_capacity(deck_starter.len());
    for card in deck_starter {
        let id_card = entities.len();
        entities.push(card);
        id_deck.push(id_card);
    }

    // Initialize map
    let (id_rooms, location) = generate_map(&mut rng, &mut entities);

    // Pre-generate monster encounters
    let mut encounter_list_normal = Vec::with_capacity(ENCOUNTER_LIST_NORMAL_CAPACITY);
    let mut encounter_list_elite = Vec::with_capacity(ENCOUNTER_LIST_ELITE_CAPACITY);
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
        id_pile_draw: Vec::with_capacity(64),
        id_hand: Vec::with_capacity(MAX_SIZE_HAND),
        id_pile_discard: Vec::with_capacity(64),
        id_pile_exhaust: Vec::with_capacity(32),
        id_card_target: None,
        id_relics,
        id_rooms,
        location,
        encounter_list_normal,
        encounter_list_elite,
        encounter_boss,
        effect_queue,
        card_last_drawn: None,
        this_turn_discards: 0,
        this_turn_attacks_played: 0,
        this_combat_damage_instances_taken: 0,
        escaped_this_combat: false,
        event_chance_monster: EVENT_CHANCE_MONSTER_BASE,
        event_chance_shop: EVENT_CHANCE_SHOP_BASE,
        event_chance_treasure: EVENT_CHANCE_TREASURE_BASE,
        potion_drop_mod: 0,
        id_card_discover: Vec::with_capacity(3),
        id_card_nightmare: None,
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
