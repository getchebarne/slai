// Entity
pub const MAX_MOVE_HISTORY: usize = 64;
// Per-card effect array cap; bump if any card legitimately exceeds 8
pub const MAX_EFFECTS_PER_CARD: usize = 8;
// Per-move effect array cap; Hexaghost's Inferno is the 7-effect maximum
pub const MAX_EFFECTS_PER_MOVE: usize = 7;
// Per-monster move array cap; Hexaghost and The Guardian have 7
pub const MAX_MOVES_PER_MONSTER: usize = 7;
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
pub const FACTOR_WEAK_PAPER_KRANE: f32 = 0.60;
pub const FACTOR_VULN: f32 = 1.50;
pub const FACTOR_VULN_ODD_MUSHROOM: f32 = 1.25;
pub const FACTOR_FRAIL: f32 = 0.75;
pub const MODE_SHIFT_INCREASE_PER_CYCLE: i16 = 10;
pub const HEXAGHOST_DIVIDER_HITS: u8 = 6;
pub const MAX_MONSTERS: usize = 5;
pub const MAX_EVENT_ROLLS: usize = 4;
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
pub const FACTOR_NUM_ELITE_A1_MULT: f32 = 1.6;
pub const FACTOR_NUM_EVENT: f32 = 0.22;
pub const FACTOR_NUM_SHOP: f32 = 0.05;
pub const MAP_ROW_TREASURE: usize = 8;
pub const UNKNOWN_CHANCE_BASE_MONSTER: f32 = 0.10;
pub const UNKNOWN_CHANCE_BASE_SHOP: f32 = 0.03;
pub const UNKNOWN_CHANCE_BASE_TREASURE: f32 = 0.02;
pub const EVENT_SPECIAL_CHANCE: f32 = 0.25;

// Chest size roll thresholds
pub const CHEST_SMALL_PCT: u8 = 50;
pub const CHEST_SMALL_PLUS_MEDIUM_PCT: u8 = 83;
pub const CHEST_SMALL_GOLD_CHANCE: u8 = 50;
pub const CHEST_SMALL_GOLD_BASE: u16 = 25;
pub const CHEST_SMALL_TH_COMMON: u8 = 75;
pub const CHEST_SMALL_TH_UNCOMMON: u8 = 100;
pub const CHEST_MEDIUM_GOLD_CHANCE: u8 = 35;
pub const CHEST_MEDIUM_GOLD_BASE: u16 = 50;
pub const CHEST_MEDIUM_TH_COMMON: u8 = 35;
pub const CHEST_MEDIUM_TH_UNCOMMON: u8 = 85;
pub const CHEST_LARGE_GOLD_CHANCE: u8 = 50;
pub const CHEST_LARGE_GOLD_BASE: u16 = 75;
pub const CHEST_LARGE_TH_COMMON: u8 = 0;
pub const CHEST_LARGE_TH_UNCOMMON: u8 = 75;

// Cumulative thresholds for relic-tier roll (elite reward, random grant)
pub const RELIC_TIER_TH_COMMON: u8 = 50;
pub const RELIC_TIER_TH_UNCOMMON: u8 = 83;

// Encounter sequence sizes
pub const NUM_ENCOUNTERS_WEAK: usize = 3;
pub const NUM_ENCOUNTERS_HARD: usize = MAP_HEIGHT - NUM_ENCOUNTERS_WEAK;
pub const NUM_ENCOUNTERS_ELITE: usize = 10;
pub const ENCOUNTER_POOL_CAPACITY_NORMAL: usize = NUM_ENCOUNTERS_WEAK + 1 + NUM_ENCOUNTERS_HARD;
pub const ENCOUNTER_POOL_CAPACITY_ELITE: usize = NUM_ENCOUNTERS_ELITE;

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

// Shop pricing — cards: base × U[0.9, 1.1], colorless × 1.2
pub const SHOP_PRICE_CARD_COMMON: u16 = 50;
pub const SHOP_PRICE_CARD_UNCOMMON: u16 = 75;
pub const SHOP_PRICE_CARD_RARE: u16 = 150;
pub const SHOP_PRICE_COLORLESS_NUMER: u16 = 6;
pub const SHOP_PRICE_COLORLESS_DENOM: u16 = 5;
pub const SHOP_PRICE_CARD_VARIANCE_MIN: f32 = 0.9;
pub const SHOP_PRICE_CARD_VARIANCE_MAX: f32 = 1.1;

// Shop pricing — potions and relics: base × U[0.95, 1.05]
pub const SHOP_PRICE_POTION_COMMON: u16 = 50;
pub const SHOP_PRICE_POTION_UNCOMMON: u16 = 75;
pub const SHOP_PRICE_POTION_RARE: u16 = 100;
pub const SHOP_PRICE_RELIC_COMMON: u16 = 150;
pub const SHOP_PRICE_RELIC_UNCOMMON: u16 = 250;
pub const SHOP_PRICE_RELIC_RARE: u16 = 300;
pub const SHOP_PRICE_RELIC_SHOP: u16 = 150;
pub const SHOP_PRICE_RELIC_POTION_VARIANCE_MIN: f32 = 0.95;
pub const SHOP_PRICE_RELIC_POTION_VARIANCE_MAX: f32 = 1.05;

pub const SHOP_PURGE_COST_BASE: u16 = 75;
pub const SHOP_PURGE_COST_INCREMENT: u16 = 25;

// A16+ markup: card/relic/potion prices × 11/10 rounded half-up; purge cost is exempt
pub const ASCENSION_SHOP_PRICE_BUMP_LEVEL: u8 = 16;
pub const ASCENSION_SHOP_PRICE_BUMP_NUMER: u16 = 11;
pub const ASCENSION_SHOP_PRICE_BUMP_DENOM: u16 = 10;

// Shop inventory composition
pub const SHOP_SLOTS_CARD_COLORED: usize = 5;
pub const SHOP_SLOTS_CARD_COLORLESS: usize = 2;
pub const SHOP_SLOTS_CARD_TOTAL: usize = SHOP_SLOTS_CARD_COLORED + SHOP_SLOTS_CARD_COLORLESS;
pub const SHOP_SLOTS_RELIC: usize = 3;
pub const SHOP_SLOTS_POTION: usize = 3;

// Colored-card rarity weights (cumulative < thresholds)
pub const SHOP_CARD_TH_COMMON: u8 = 60;
pub const SHOP_CARD_TH_UNCOMMON: u8 = 97;

// Relic-tier weights for the 2 non-shop relic slots
pub const SHOP_RELIC_TH_COMMON: u8 = 48;
pub const SHOP_RELIC_TH_UNCOMMON: u8 = 82;

// Initial capacity for the per-handler effect builder on GameState
pub const MAX_EFFECTS_PER_HANDLER: usize = 32;

// Initial capacity for the per-resolve candidate buffer on GameState
pub const MAX_CANDIDATES: usize = 128;
