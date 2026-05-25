// Entity
pub const MAX_MOVE_HISTORY: usize = 64;
pub const STARTING_GOLD: u16 = 99;
pub const MAX_GOLD: u16 = 9999;

pub const GOLD_MONSTER_MIN: u16 = 10;
pub const GOLD_MONSTER_MAX: u16 = 20;
pub const GOLD_ELITE_MIN: u16 = 25;
pub const GOLD_ELITE_MAX: u16 = 35;
pub const GOLD_BOSS_MIN: u16 = 95;
pub const GOLD_BOSS_MAX: u16 = 105;

// Combat
pub const MAX_SIZE_HAND: usize = 10;
pub const MAX_SIZE_DECK: usize = 99;
pub const MAX_ENTITIES: usize = 1024;
pub const CARDS_DRAWN_PER_TURN: u16 = 5;
pub const MAX_COMBAT_CARD_REWARD: usize = 3;
pub const FACTOR_WEAK: f32 = 0.75;
pub const FACTOR_VULN: f32 = 1.50;
pub const FACTOR_FRAIL: f32 = 0.75;
pub const MODE_SHIFT_INCREASE_PER_CYCLE: i16 = 10;
pub const HEXAGHOST_DIVIDER_HITS: u8 = 6;
pub const MAX_MONSTERS: usize = 5;
pub const MAX_BLOCK: u16 = 999;
pub const NIGHTMARE_COPIES: u8 = 3;

// Card rewards
pub const CARD_REWARD_ROLL_OFFSET_BASE: i8 = 5;
pub const CARD_REWARD_ROLL_OFFSET_MIN: i8 = -40;
pub const CARD_REWARD_ROLL_CHANCE_RARE: i32 = 3;
pub const CARD_REWARD_ROLL_CHANCE_UNCOMMON: i32 = 40;

// Map
pub const MAP_HEIGHT: usize = 15;
pub const MAP_WIDTH: usize = 7;
pub const PATH_DENSITY: usize = 6;
pub const ANCESTOR_GAP_MIN: usize = 3;

pub const FACTOR_NUM_REST_SITE: f32 = 0.12;
pub const FACTOR_NUM_ELITE: f32 = 0.08;
pub const FACTOR_NUM_EVENT: f32 = 0.22;
pub const FACTOR_NUM_SHOP: f32 = 0.05;

pub const MAP_ROW_TREASURE: usize = 8;

pub const EVENT_CHANCE_BASE_MONSTER: f32 = 0.10;
pub const EVENT_CHANCE_BASE_SHOP: f32 = 0.03;
pub const EVENT_CHANCE_BASE_TREASURE: f32 = 0.02;

// Chest size roll thresholds
pub const CHEST_SMALL_PCT: u8 = 50;
pub const CHEST_SMALL_PLUS_MEDIUM_PCT: u8 = 83;

#[derive(Debug, Clone, Copy)]
pub struct ChestParams {
    pub gold_chance: u8,
    pub gold_base: u16,
    pub th_common: u8,
    pub th_uncommon: u8,
}

pub const CHEST_SMALL: ChestParams = ChestParams {
    gold_chance: 50,
    gold_base: 25,
    th_common: 75,
    th_uncommon: 100,
};
pub const CHEST_MEDIUM: ChestParams = ChestParams {
    gold_chance: 35,
    gold_base: 50,
    th_common: 35,
    th_uncommon: 85,
};
pub const CHEST_LARGE: ChestParams = ChestParams {
    gold_chance: 50,
    gold_base: 75,
    th_common: 0,
    th_uncommon: 75,
};

// Cumulative `<` thresholds for relic-tier roll.
// roll < th_common → COMMON; th_common..th_uncommon → UNCOMMON; else → RARE
pub const ELITE_TH_COMMON: u8 = 50;
pub const ELITE_TH_UNCOMMON: u8 = 83;

// Encounter sequence sizes
pub const NUM_ENCOUNTERS_WEAK: usize = 3;
pub const NUM_ENCOUNTERS_HARD: usize = MAP_HEIGHT - NUM_ENCOUNTERS_WEAK;
pub const NUM_ENCOUNTERS_ELITE: usize = 10;
pub const ENCOUNTER_LIST_CAPACITY_NORMAL: usize = NUM_ENCOUNTERS_WEAK + 1 + NUM_ENCOUNTERS_HARD;
pub const ENCOUNTER_LIST_CAPACITY_ELITE: usize = NUM_ENCOUNTERS_ELITE;

// Rest site
pub const REST_SITE_HEAL_FACTOR: f32 = 0.30;

// Potions
pub const POTION_SLOTS_DEFAULT: u8 = 3;
pub const POTION_SLOTS_DEFAULT_A11: u8 = 2;
pub const POTION_SLOTS_MAX: usize = 5;

// Potion drop swing: chance = base + mod; +10 on miss, -10 on hit
pub const POTION_DROP_CHANCE_BASE: i8 = 40;
pub const POTION_DROP_CHANCE_MOD_HIT: i8 = -10;
pub const POTION_DROP_CHANCE_MOD_MISS: i8 = 10;
pub const POTION_DROP_CHANCE_MOD_MIN: i8 = -30;
pub const POTION_DROP_CHANCE_MOD_MAX: i8 = 60;

// Potion rarity roll thresholds
pub const POTION_TH_COMMON: u8 = 65;
pub const POTION_TH_UNCOMMON: u8 = 90;

// Discovery: number of card options offered
pub const DISCOVER_PICK_COUNT: u8 = 3;

// Initial capacity for the per-handler effect builder on GameState
pub const MAX_EFFECTS_PER_HANDLER: usize = 32;

// Initial capacity for the per-resolve candidate buffer on GameState. Sized
// for CandidatePool::DeckFiltered worst case (full deck), with headroom
pub const MAX_CANDIDATES: usize = 128;
